use bevy::prelude::*;
use blenvy::{BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

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
