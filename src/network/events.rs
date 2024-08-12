use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::prelude::*;

use crate::network::resources::PLAYER_COLORS;

use super::resources::{LobbyConfig, LobbyInput, Settings};

#[derive(Event)]
pub struct ControllerLoadEvent {
    pub player_number: usize,
    pub player_color: Color,
}

#[derive(Event)]
pub struct LocalInputEvent(pub HashMap<usize, LobbyInput>);

pub fn connect_to_players(
    mut connect_ev: EventWriter<ControllerLoadEvent>,
    session: Res<Session<LobbyConfig>>,
    net_settings: Res<Settings>,
) {
    println!("number of players: {:?}", net_settings.players);
    let local_player = match &*session {
        Session::SyncTest(_s) => 0,
        Session::P2P(s) => s.local_player_handles()[0],
        Session::Spectator(_s) => 0,
    };
    connect_ev.send(ControllerLoadEvent {
        player_number: local_player,
        player_color: PLAYER_COLORS[local_player],
    });
}
