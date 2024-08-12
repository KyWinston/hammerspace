use std::io::{BufReader, BufWriter, Cursor};

use baby_shark::{
    decimation::{edge_decimation::ConstantErrorDecimationCriteria, prelude::EdgeDecimator},
    io::stl::{StlReader, StlWriter},
    mesh::corner_table::table::CornerTable,
};
use bevy::{
    asset::{Assets, Handle},
    color::{ColorToComponents, Srgba},
    ecs::system::{Commands, Res, ResMut},
    math::{vec3, Vec2, Vec3},
    pbr::{light_consts, DirectionalLight, DirectionalLightBundle, MaterialMeshBundle},
    prelude::default,
    render::{
        mesh::{Indices, Mesh, PrimitiveTopology, VertexAttributeValues},
        render_asset::RenderAssetUsages,
        texture::Image,
    },
    state::state::NextState,
    transform::components::Transform,
};

use avian3d::{parry::na::clamp, prelude::*};
use bevy_codex::prelude::*;
use image::{DynamicImage, ImageBuffer, Rgba};
use stl_io::Vector;

use crate::game::{ImageAssets, MaterialsLoading};

pub struct TerrainMeshData {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<u32>,
}

pub type Rgba8Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub fn init_terrain(
    image_assets: Res<ImageAssets>,
    images: Res<Assets<Image>>,
    meshes: ResMut<Assets<Mesh>>,
    mat_handle: ResMut<MaterialsLoading>,
    state: ResMut<NextState<UiState>>,
    commands: Commands,
) {
    load_terrain(
        image_assets,
        images,
        meshes,
        mat_handle,
        commands,
        "terrain_flat".to_string(),
        Vec2::new(0.0, 0.0),
        state,
    );
}

pub fn load_terrain(
    image_assets: Res<ImageAssets>,
    images: Res<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mat_handle: ResMut<MaterialsLoading>,
    mut commands: Commands,
    image_name: String,
    coordinates: Vec2,
    mut state: ResMut<NextState<UiState>>,
) {
    let terrain_scale = vec3(50.0 * 3.0 / 2.0 * f32::sqrt(3.0), 8.0, 50.0);

    let terrain_image: &Image = images
        .get(&image_assets.0.get(&image_name).unwrap().clone())
        .unwrap();

    let greyscale_image: image::ImageBuffer<image::LumaA<u8>, Vec<u8>> = DynamicImage::from(
        Rgba8Image::from_raw(
            terrain_image.width(),
            terrain_image.height(),
            terrain_image.data.clone(),
        )
        .unwrap(),
    )
    .into_luma_alpha8();

    let mut terrain_mesh_data: TerrainMeshData = TerrainMeshData {
        vertices: vec![],
        indices: vec![],
    };

    let mut triangle_number = 0;
    let gray_img_h = greyscale_image.height();
    let gray_img_w = greyscale_image.width();

    for row in 0..(gray_img_h) {
        for column in 0..(gray_img_w) {
            terrain_mesh_data.vertices.push(
                Vec3::new(
                    column as f32,
                    greyscale_image.get_pixel(column, row).0[0].into(),
                    row as f32,
                ) * terrain_scale
                    / vec3(gray_img_h as f32, 255.0, gray_img_w as f32),
            );
            let (n_row, n_col) = (&(row + 1), &(column + 1));
            if check_pixel_data_opaque(&greyscale_image, &column, &row)
                && check_pixel_data_opaque(&greyscale_image, n_col, &row)
                && check_pixel_data_opaque(&greyscale_image, &column, n_row)
                && check_pixel_data_opaque(&greyscale_image, n_col, n_row)
            {
                terrain_mesh_data.indices.extend(
                    [
                        [&column, &row, &gray_img_w],
                        [&column, n_row, &gray_img_w],
                        [n_col, &row, &gray_img_w],
                        [n_col, n_row, &gray_img_w],
                        [n_col, &row, &gray_img_w],
                        [&column, n_row, &gray_img_w],
                    ]
                    .iter()
                    .map(|f| get_index_from_pos(f[0], f[1], f[2])),
                );
                triangle_number += 2;
            }
        }
    }

    let enable_wireframe = false;

    let mut mesh = Mesh::new(
        if enable_wireframe {
            PrimitiveTopology::LineList
        } else {
            PrimitiveTopology::TriangleList
        },
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<usize> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    // let mut uv: Vec<[f32; 3]> = Vec::new();
    let mut colors: Vec<[f32; 4]> = Vec::new();

    for vertex in &terrain_mesh_data.vertices {
        vertices.push([vertex.x, vertex.y, vertex.z]);
        let color = Srgba::new(clamp(vertex.y, 0.0, 100.0) / 100.0, 0.05, 0.20, 1.0);
        let raw_float: Srgba = Srgba::from_f32_array(color.to_f32_array());
        let (raw_col, alpha) = (raw_float, raw_float.alpha);
        colors.push([raw_col.red, raw_col.green, raw_col.blue, alpha]);
    }

    if enable_wireframe {
        for i in 0..triangle_number {
            for j in &[0, 1, 1, 2, 2, 0] {
                indices.push(terrain_mesh_data.indices[i * 3 + j].try_into().unwrap());
            }
        }
    } else {
        for i in 0..triangle_number {
            for j in 0..3 {
                indices.push(terrain_mesh_data.indices[i * 3 + j].try_into().unwrap());
            }
        }
    }

    let mut stl_mesh: Vec<stl_io::Triangle> = vec![];

    for i in 0..triangle_number {
        let v = i * 3;
        let (p1, p2, p3) = (
            vertices[indices[v]],
            vertices[indices[v + 1]],
            vertices[indices[v + 2]],
        );

        let (u, v) = (
            [p2[0] - p1[0], p2[1] - p1[1], p2[2] - p1[2]],
            [p3[0] - p1[0], p3[1] - p1[1], p3[2] - p1[2]],
        );

        let normal = [
            (u[1] * v[2]) - (u[2] * v[1]),
            (u[2] * v[0]) - (u[0] * v[2]),
            (u[0] * v[1]) - (u[1] * v[0]),
        ];
        normals.push(normal);
        stl_mesh.push(stl_io::Triangle {
            normal: Vector::new(normal),
            vertices: [Vector::new(p1), Vector::new(p2), Vector::new(p3)],
        })
    }

    let mut stl_buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    stl_io::write_stl(&mut stl_buffer, stl_mesh.iter()).expect("Failure writing to mesh buffer.");

    let mut mesh_from_buffer = StlReader::new()
        .read_stl::<&[u8], CornerTable<f32>>(&mut BufReader::new(&stl_buffer.into_inner()))
        .expect("Error reading mesh buffer");

    stl_buffer = Cursor::new(Vec::new());

    let mut decimator = EdgeDecimator::new()
        .decimation_criteria(ConstantErrorDecimationCriteria::new(1.0))
        .keep_boundary(true);
    decimator.decimate(&mut mesh_from_buffer);

    let stl_writer = StlWriter::new();

    stl_writer
        .write_stl(
            &mesh_from_buffer,
            &mut BufWriter::new(&mut BufWriter::new(&mut stl_buffer)),
        )
        .expect("Failure writing mesh to second buffer");

    let indexed_mesh =
        stl_io::read_stl(&mut stl_buffer).expect("error reading mesh from second buffer");

    let vertices: Vec<[f32; 3]> = indexed_mesh
        .vertices
        .into_iter()
        .map(|x| Into::<[f32; 3]>::into(x))
        .collect();
    let mut indices: Vec<u32> = Vec::new();

    for face in indexed_mesh.faces {
        for vertex in face.vertices {
            indices.push(vertex.try_into().unwrap());
        }
    }

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::Float32x3(vertices.clone()),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        VertexAttributeValues::Float32x4(colors),
    );

    let mut uvs: Vec<[f32; 2]> = vec![];
    for count in vertices {
        uvs.push([
            Vec2::length(Vec2::new(count[0], count[1])),
            Vec2::length(Vec2::new(count[2], count[1])),
        ]);
    }
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float32x3(normals),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::Float32x2(uvs));
    mesh.insert_indices(Indices::U32(indices));
    mesh.generate_tangents()
        .expect("tangents couldn't be generated");

    let terrain_mesh_handle: Handle<Mesh> = meshes.add(mesh.clone());
    let paint_material = mat_handle.0[0].clone();

    let mut initial_position: Vec3 = Vec3::new(
        -terrain_scale.x / 2.0,
        -terrain_scale.y,
        -terrain_scale.z / 2.0,
    );

    if terrain_scale.z as i32 % 2 == 1 {
        initial_position += Vec3::new(terrain_scale.x / 2.0, 0.0, 0.0)
    }

    let new_position = initial_position
        + Vec3::new(
            terrain_scale.x * coordinates.x,
            0.0,
            terrain_scale.z * coordinates.y,
        );

    commands
        .spawn((
            // Showcase,
            MaterialMeshBundle {
                mesh: terrain_mesh_handle,
                material: paint_material,
                transform: Transform::from_xyz(new_position.x, new_position.y, new_position.z),
                ..default()
            },
            RigidBody::Static,
        ))
        .insert(Collider::trimesh_from_mesh(&mesh).unwrap());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::CIVIL_TWILIGHT,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    state.set(UiState::Hud);
}

fn check_pixel_data_opaque(
    buffer: &image::ImageBuffer<image::LumaA<u8>, Vec<u8>>,
    x: &u32,
    y: &u32,
) -> bool {
    if x < &buffer.width() && y < &buffer.height() {
        buffer.get_pixel(*x, *y).0[1] > 0
    } else {
        false
    }
}

fn get_index_from_pos(x: &u32, y: &u32, width: &u32) -> u32 {
    y * width + x
}
