use std::f32::consts::PI;

use bevy::{color::palettes::css::SILVER, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::Image}};
use bevy::prelude::*;
use bevy_mod_picking::{backends::raycast::RaycastPickable, events::{Down, Pointer}, prelude::{ListenerMut, On}};
use bevy_quill::View;
use bevy_quill_obsidian::viewport;

use crate::{components::Shape, resources::{PreviewEntities, SelectedShape}, MainDock, X_EXTENT};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let shapes = [
        meshes.add(Cuboid::default()),
        meshes.add(Capsule3d::default()),
        meshes.add(Torus::default()),
        meshes.add(Cylinder::default()),
        meshes.add(Sphere::default().mesh().ico(5).unwrap()),
        meshes.add(Sphere::default().mesh().uv(32, 18)),
    ];

    let num_shapes = shapes.len();
    let debug_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(uv_debug_texture())),
        ..default()
    });
    let shapes_parent = commands
        .spawn((
            SpatialBundle { ..default() },
            // BackdropPickable,
            On::<Pointer<Down>>::run(
                |mut event: ListenerMut<Pointer<Down>>,
                 shapes: Query<&Shape>,
                 mut selection: ResMut<SelectedShape>| {
                    if shapes.get(event.target).is_ok() {
                        selection.0 = Some(event.target);
                        // println!("Pointer down on shape {:?}", event.target);
                    } else {
                        selection.0 = None;
                        // println!("Pointer down on backdrop {:?}", event.target);
                    }
                    event.stop_propagation();
                },
            ),
        ))
        .id();

    for (i, shape) in shapes.into_iter().enumerate() {
        commands
            .spawn((
                PbrBundle {
                    mesh: shape,
                    material: debug_material.clone(),
                    transform: Transform::from_xyz(
                        -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                        2.0,
                        0.0,
                    )
                    .with_rotation(Quat::from_rotation_x(PI / 4.)),
                    ..default()
                },
                Shape,
                // PickableBundle::default(),
                RaycastPickable,
            ))
            .set_parent(shapes_parent);
    }

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            // intensity: 9000.0,
            intensity: 10000000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
        material: materials.add(Color::from(SILVER)),
        ..default()
    });
}

pub fn start_editor(camera: In<Entity>, mut commands: Commands) {
    commands.spawn(MainDock(*camera).to_root());
}

pub fn setup_ui(mut commands: Commands) -> Entity {
    commands
        .spawn((Camera2dBundle {
            camera: Camera {
                // HUD goes on top of 3D
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            camera_2d: Camera2d {},
            ..default()
        },))
        .id()
}

pub fn enter_preview_mode(mut commands: Commands) {
    let camera = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 6., 12.0)
                    .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
                ..default()
            },
            viewport::ViewportCamera,
            RaycastPickable,
            // BackdropPickable,
        ))
        .id();

    // let overlay = commands.spawn(TransformOverlayDemo.to_root()).id();
    let overlay = commands.spawn_empty().id();
    commands.insert_resource(PreviewEntities {
        camera,
        _overlay: overlay,
    });
}

pub fn exit_preview_mode(mut commands: Commands, preview: Res<PreviewEntities>) {
    commands.entity(preview.camera).despawn();
    // commands.add(DespawnViewRoot::new(preview.overlay));
    commands.remove_resource::<PreviewEntities>()
}

/// Creates a colorful test pattern
pub fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    )
}

pub fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() / 2.);
    }
}

pub fn close_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
