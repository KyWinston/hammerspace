use bevy::prelude::*;
use bevy_ggrs::GgrsConfig;
use serde::{Deserialize, Serialize};

pub type LobbyConfig = GgrsConfig<LobbyInput>;

const BLUE: Color = Color::srgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::srgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::srgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::srgb(0.35, 0.7, 0.35);
pub const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];


#[derive(Resource)]
pub struct NetworkStatsTimer(pub Timer);

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct LobbyInput(pub u8);

#[derive(Resource, Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub local_port: u16,
    pub players: Vec<String>,
}
