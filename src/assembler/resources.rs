use super::AssetLoadState;
use bevy::{asset::Handle, gltf::Gltf, prelude::*, utils::HashMap};
use blenvy::GameWorldTag;

#[derive(Resource)]
pub struct LoadingTextures(pub Vec<Sprite>);

#[derive(Resource)]
pub(crate) struct SessionAssets(
    pub HashMap<String, String>,
    pub HashMap<String, String>,
    pub HashMap<String, String>,
    pub HashMap<String, String>,
);

#[derive(Resource, Default)]
pub(crate) struct ImageAssets(pub HashMap<String, Sprite>);

#[derive(Resource, Default)]
pub(crate) struct MeshAssets(pub HashMap<String, Handle<Gltf>>);

#[derive(Resource, Default)]
pub(crate) struct PreparedScenes(pub HashMap<String, Handle<Gltf>>);

#[derive(Resource, Default)]
pub(crate) struct ImageAssetsLoading(pub Vec<Sprite>);

#[derive(Resource, Default)]
pub(crate) struct MeshAssetsLoading(pub Vec<Handle<Gltf>>);

pub(crate) fn init_resources(
    mut commands: Commands,
    session_assets: Res<SessionAssets>,
    mut mesh_assets: ResMut<MeshAssets>,
    mut image_assets: ResMut<ImageAssets>,
    mut scenes: ResMut<PreparedScenes>,
    server: Res<AssetServer>,
) {
    info!("initializing resources");
    info!("initializing sprites");

    //scene
    scenes.0.extend(
        session_assets
            .0
            .iter()
            .map(|f| (f.0.to_string(), server.load(f.1.to_string() + ".gltf"))),
    );

    for map in [
        ["uv_color", "uv_canvas"],
        ["occlusion", "occlusion"],
        ["normals", "normal_sheet"],
        ["mask", "uv_sheet"],
        ["volume", "volume"],
    ] {
        //sprite_sheets
        image_assets.0.extend(session_assets.1.iter().map(|f| {
            (
                map[0].to_string(),
                Sprite {
                    image: server.load("images/sprites/".to_owned() + f.1 + "/" + map[1] + ".png"),
                    ..default()
                },
            )
        }));
    }
    info!("initializing images");

    //still images
    image_assets.0.extend(session_assets.2.iter().map(|f| {
        (
            f.0.to_string(),
            Sprite {
                image: server.load("images/".to_owned() + f.1 + ".png"),
                ..default()
            },
        )
    }));

    //meshes
    mesh_assets.0.extend(
        session_assets
            .3
            .iter()
            .map(|f| (f.0.to_string(), server.load(f.1.to_string() + ".gltf"))),
    );

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

pub(crate) fn check_assets_ready(
    mut commands: Commands,
    world: Query<&GameWorldTag>,
    mut asset_state_next: ResMut<NextState<AssetLoadState>>,
    server: Res<AssetServer>,
    _images: Res<ImageAssets>,
    image_assets_loading: Res<ImageAssetsLoading>,
    mesh_assets_loading: Res<MeshAssetsLoading>,
) {
    if world.get_single().is_ok() {
        info!("checking assets");
        let mut not_loaded_count: i64 = 0;
        let mut load_failure = false;
        info!("checking images");
        for sprite in &image_assets_loading.0 {
            match server.get_load_state(&sprite.image.clone()).unwrap() {
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
                bevy::asset::LoadState::Failed(err) => {
                    load_failure = true;
                    error!("Mesh failed to load: {:?}", err);
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
            asset_state_next.set(is_loaded_without_failure);
        }
    }
}
