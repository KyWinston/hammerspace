use bevy::{
    app::{App, Plugin},
    log::info,
    state::state::OnEnter,
};
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;
use systems::init_terrain;

use super::AssetLoadState;

pub mod systems;

#[derive(Parser, ConsoleCommand)]
#[command(name = "terrain")]
struct TerrainCommand {
    msg: String,
    num: u32,
}

fn load_terrain_command(mut log: ConsoleCommand<TerrainCommand>) {
    if let Some(Ok(TerrainCommand { msg, num })) = log.take() {
        info!(msg);
        info!(num);
    }
}

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoadState::Loaded), init_terrain);
        app.add_console_command::<TerrainCommand, _>(load_terrain_command);
    }
}
