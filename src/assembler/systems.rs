use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

use crate::interact::components::Actor;

use super::{
    components::{BlueprintInstanceReady, GameWorldTag, Sky},
    events::{BlueprintReadyEvent, LevelLoadedEvent, PrepareLevelEvent},
};

pub fn setup_blueprints(
    mut level_ev: EventReader<PrepareLevelEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in level_ev.read() {
        commands.spawn((
            SceneRoot(
                asset_server
                    .load(GltfAssetLabel::Scene(0).from_asset(format!("levels/{}.glb", ev.0))),
            ),
            GameWorldTag,
        ));
    }
}

pub fn spawn_blueprint<'a>(
    commands: &'a mut Commands,
    name: String,
    location: Transform,
    asset_server: Res<AssetServer>,
) -> EntityCommands<'a> {
    commands.spawn((
        SceneRoot(
            asset_server
                .load(GltfAssetLabel::Scene(0).from_asset(format!("blueprints/{}.glb", name))),
        ),
        Actor,
        Name::from(name),
        location,
    ))
}

pub(crate) fn on_level_loaded(
    trigger: Trigger<OnAdd, BlueprintInstanceReady>,
    mut commands: Commands,
    mut level_ev: EventWriter<LevelLoadedEvent>,
    levels: Query<Entity, With<GameWorldTag>>,
    sun_light: Single<Entity, With<DirectionalLight>>,
    sky: Single<Entity, With<Sky>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for level in levels.iter() {
        if trigger.target() == level {
            level_ev.write(LevelLoadedEvent(trigger.target()));
        }
    }
    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: 3.0,
        ..default()
    }
    .build();
    commands.entity(*sun_light).insert(cascade_shadow_config);
    commands
        .entity(*sky)
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("888888").unwrap().into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })));
}

pub(crate) fn on_blueprint_complete(
    trigger: Trigger<OnAdd, BlueprintInstanceReady>,
    mut ev: EventWriter<BlueprintReadyEvent>,
    levels: Query<Entity, With<GameWorldTag>>,
) {
    for level in levels.iter() {
        if trigger.target() == level {
            return;
        }
    }
    ev.write(BlueprintReadyEvent(trigger.target()));
}
