use super::AssetLoadState;
use bevy::{gltf::Gltf, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct LoadingTextures(pub Vec<Handle<Image>>);

#[derive(Resource)]
pub struct SessionAssets(
    pub HashMap<String, String>,
    pub HashMap<String, String>,
    pub HashMap<String, String>,
    pub HashMap<String, String>,
);

#[derive(Resource, Default)]
pub struct ImageAssets(pub HashMap<String, Handle<Image>>);

#[derive(Resource, Default)]
pub(crate) struct MeshAssets(pub HashMap<String, Handle<Gltf>>);

#[derive(Resource, Default)]
pub(crate) struct PreparedScenes(pub HashMap<String, Handle<Gltf>>);

#[derive(Resource, Default)]
pub(crate) struct ImageAssetsLoading(pub Vec<Handle<Image>>);

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
                server.load("images/sprites/".to_owned() + f.1 + "/" + map[1] + ".png"),
            )
        }));
    }
    info!("initializing images");

    //still images
    image_assets.0.extend(session_assets.2.iter().map(|f| {
        (
            f.0.to_string(),
            server.load("images/".to_owned() + f.1 + ".png"),
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
        info!("resolving load");
        asset_state_next.set(is_loaded_without_failure);
    }
}
