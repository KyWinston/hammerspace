use bevy::prelude::*;
use blenvy::{BlueprintInfo, Dynamic, GameWorldTag, HideUntilReady, SpawnBlueprint};

use crate::interact::components::Actor;

use super::events::PrepareLevelEvent;

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
        BlueprintInfo {
            name: name.clone(),
            path: format!("blueprints/{}.glb", name),
        },
        Actor,
        Dynamic,
        Name::from(format!("{}", name)),
        location,
    ))
}
