use bevy::prelude::*;
use blenvy::{BlueprintInfo, GameWorldTag, HideUntilReady, SpawnBlueprint};

pub fn setup_blueprints(mut commands: Commands) {
    commands.spawn((
        BlueprintInfo::from_path("levels/concrete_island.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
    ));
}
