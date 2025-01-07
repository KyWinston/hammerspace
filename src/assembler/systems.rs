use bevy::prelude::*;
use blenvy::{
    AddToGameWorld, BlueprintInfo, Dynamic, GameWorldTag, HideUntilReady, SpawnBlueprint,
};
use rand::Rng;

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

pub fn spawn_actor<'a>(commands: &'a mut Commands, name: String) -> EntityCommands<'a> {
    let mut rng = rand::thread_rng();
    let range = 1.5;
    let x: f32 = rng.gen_range(-range..range);
    let y: f32 = rng.gen_range(-range..range);

    let name_index: u64 = rng.gen();

    commands.spawn((
        BlueprintInfo {
            name: name.clone(),
            path: format!("blueprints/{}.glb", name),
        },
        Dynamic,
        Name::from(format!("test {}", name_index)),
        HideUntilReady,
        AddToGameWorld,
        Transform::from_xyz(x, 2.0, y),
    ))
}

