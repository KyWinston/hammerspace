use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};
use blenvy::{BlueprintInfo, BlueprintInstanceReady, GameWorldTag, HideUntilReady, SpawnBlueprint};

use crate::interact::components::Actor;

use super::{
    components::Sky,
    events::{BlueprintReadyEvent, LevelLoadedEvent, PrepareLevelEvent},
};

pub fn setup_blueprints(mut level_ev: EventReader<PrepareLevelEvent>, mut commands: Commands) {
    for ev in level_ev.read() {
        commands.spawn((
            BlueprintInfo::from_path(format!("levels/{}.glb", ev.0).as_str()),
            SpawnBlueprint,
            HideUntilReady,
            GameWorldTag,
        ));
    }
}

pub fn spawn_actor<'a>(
    commands: &'a mut Commands,
    name: String,
    location: Transform,
) -> EntityCommands<'a> {
    commands.spawn((
        SpawnBlueprint,
        BlueprintInfo {
            name: name.clone(),
            path: format!("blueprints/{}.glb", name),
        },
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
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    for level in levels.iter() {
        if trigger.entity() == level {
            level_ev.send(LevelLoadedEvent(trigger.entity()));
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
        if trigger.entity() == level {
            return;
        }
    }
    ev.send(BlueprintReadyEvent(trigger.entity()));
}
