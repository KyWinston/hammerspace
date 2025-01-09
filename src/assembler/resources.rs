use super::AssetLoadState;
use bevy::{asset::Handle, gltf::Gltf, prelude::*, utils::HashMap};
use blenvy::GameWorldTag;
use iyes_progress::ProgressEntry;

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
    commands.insert_resource(ImageAssetsLoading(loading_images.clone()));
    commands.insert_resource(MeshAssetsLoading(loading_meshes.clone()));
}

pub(crate) fn check_assets_ready(
    world: Query<&GameWorldTag>,
    progress: ProgressEntry<AssetLoadState>,
    mut initted: Local<bool>,
    server: Res<AssetServer>,
    _images: Res<ImageAssets>,
    image_assets_loading: Res<ImageAssetsLoading>,
    mesh_assets_loading: Res<MeshAssetsLoading>,
) {
    if world.get_single().is_ok() {
        info!("checking assets");
        info!("checking images");
        if !*initted {
            progress.set_total(
                image_assets_loading.0.len() as u32 + mesh_assets_loading.0.len() as u32,
            );
            *initted = true;
        }
        for sprite in &image_assets_loading.0 {
            match server.get_load_state(&sprite.image.clone()).unwrap() {
                bevy::asset::LoadState::Failed(err) => {
                    error!("Image failed to load: {:?}", err);
                }
                bevy::asset::LoadState::Loaded => {
                    progress.add_done(1);
                }
                _ => {}
            }
        }
        info!("checking meshes");

        for mesh_and_scene in &mesh_assets_loading.0 {
            match server.get_load_state(&mesh_and_scene.clone()).unwrap() {
                bevy::asset::LoadState::Failed(err) => {
                    error!("Mesh failed to load: {:?}", err);
                }
                bevy::asset::LoadState::Loaded => {
                    progress.add_done(1);
                }
                _ => {}
            }
        }
    }
}
