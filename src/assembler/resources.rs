use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

use super::AssetLoadState;

#[derive(Resource)]
pub struct NextLevel(pub Handle<Gltf>);

#[derive(Resource)]
pub struct LoadingTextures(pub Vec<Handle<Image>>);

#[derive(Resource, Default)]
pub struct ImageAssets(pub HashMap<String, Handle<Image>>);

#[derive(Resource, Default)]
pub struct MeshAssets(pub HashMap<String, Handle<Gltf>>);

#[derive(Resource, Default)]
pub struct ImageAssetsLoading(pub Vec<Handle<Image>>);

#[derive(Resource, Default)]
pub struct MeshAssetsLoading(pub Vec<Handle<Gltf>>);

pub fn init_resources(
    mut commands: Commands,
    mut mesh_assets: ResMut<MeshAssets>,
    mut image_assets: ResMut<ImageAssets>,
    server: Res<AssetServer>,
) {
    info!("initializing resources");
    info!("initializing sprites");
    //sprite_sheets
    image_assets.0.extend(
        ([
            ("mask", "sprites/warrior/uv_sheet"),
            ("normals", "sprites/warrior/normal_sheet_t"),
            ("occlusion", "sprites/warrior/occlusion"),
            ("volume", "sprites/warrior/volume"),
            ("uv_color", "sprites/warrior/uv_canvas"),

        ])
        .map(|f| {
            (
                f.0.to_string(),
                server.load("images/".to_owned() + f.1 + ".png"),
            )
        }),
    );
    info!("initializing images");
    //still images
    image_assets.0.extend(
        ([
            ["grunge", "textures/brush_grunge"],
            ["grunge_normal", "textures/brush_grunge_normal"],
        ]
        .into_iter())
        .map(|f| (f[0].to_string(), server.load(f[1].to_owned() + ".png"))),
    );

    //meshes
    mesh_assets.0.insert(
        "level".to_string(),
        server.load("levels/concrete_island.gltf"),
    );
    info!("initializing meshes");
    mesh_assets
        .0
        .insert("Furnace".to_string(), server.load("objects/furnace.gltf"));
    mesh_assets
        .0
        .insert("monster_bed".to_string(), server.load("objects/bed.gltf"));

    let mut loading_images = Vec::new();
    let mut loading_meshes = Vec::new();

    for image in &image_assets.0 {
        loading_images.push(image.1.clone())
    }

    for mesh in &mesh_assets.0 {
        loading_meshes.push(mesh.1.clone());
    }
    commands.insert_resource(ImageAssetsLoading(loading_images));
    commands.insert_resource(MeshAssetsLoading(loading_meshes));
}

pub fn check_assets_ready(
    mut commands: Commands,
    mut asset_state_next: ResMut<NextState<AssetLoadState>>,
    server: Res<AssetServer>,
    _images: Res<ImageAssets>,
    image_assets_loading: Res<ImageAssetsLoading>,
    mesh_assets_loading: Res<MeshAssetsLoading>,
) {
    info!("checking assets");
    let mut not_loaded_count: i64 = 0;
    let mut load_failure = false;
    info!("checking images");
    for image_handle in &image_assets_loading.0 {
        match server.get_load_state(&image_handle.clone()).unwrap() {
            bevy::asset::LoadState::Failed(_) => {
                load_failure = true;
                error!("Image failed to load");
            }
            bevy::asset::LoadState::Loaded => {}
            _ => {
                not_loaded_count += 1;
            }
        }
    }
    info!("checking meshes");
    for mesh_and_scene in &mesh_assets_loading.0 {
        match server.get_load_state(&mesh_and_scene.clone()).unwrap() {
            bevy::asset::LoadState::Failed(_) => {
                load_failure = true;
                error!("Mesh failed to load");
            }
            bevy::asset::LoadState::Loaded => {}
            _ => {
                not_loaded_count += 1;
            }
        }
    }

    let is_loaded = if not_loaded_count > 0 {
        AssetLoadState::Loading
    } else {
        AssetLoadState::Loaded
    };

    let is_loaded_without_failure = if !load_failure {
        is_loaded
    } else {
        AssetLoadState::Failed
    };

    if is_loaded_without_failure == AssetLoadState::Loaded {
        commands.remove_resource::<ImageAssetsLoading>();
        commands.remove_resource::<MeshAssetsLoading>();
        info!("resolving load");
        asset_state_next.set(is_loaded_without_failure);
    }
}

// pub fn substitute_prefab(
//     prefab: (String, Mesh),
//     gltf: Res<MeshAssets>,
//     mut meshes: ResMut<Assets<Mesh>>,
// ) -> Handle<Mesh> {

//     meshes.add(prefab.1)
// }
