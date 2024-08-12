use crate::network::NetState;
use bevy::{prelude::*, utils::FixedState};
use bevy_ggrs::{
    ggrs::{DesyncDetection, GgrsEvent, PlayerType, SessionBuilder, UdpNonBlockingSocket},
    LocalInputs, Session,
};
use std::hash::{BuildHasher, Hash, Hasher};

use super::{
    events::LocalInputEvent,
    resources::{LobbyConfig, NetworkStatsTimer, Settings},
};

pub fn start_match_session(
    // mut commands: Commands,
    mut session: ResMut<Session<LobbyConfig>>,
    mut state: ResMut<NextState<NetState>>,
) {
    match session.as_mut() {
        Session::P2P(s) => {
            if s.num_players() > 1 {
                for event in s.events() {
                    match event {
                        GgrsEvent::Synchronized { .. } => {
                            println!("connected");
                            state.set(NetState::Connected);
                            // commands.spawn(Hud);
                        }
                        _ => (),
                    }
                }
            } else {
                state.set(NetState::Offline);
                // commands.spawn(Hud);
            }
        }
        _ => (),
    }
}

pub fn print_events_system(mut session: ResMut<Session<LobbyConfig>>) {
    match session.as_mut() {
        Session::P2P(s) => {
            for event in s.events() {
                match event {
                    GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
                        warn!("GGRS event: {event:?}")
                    }
                    GgrsEvent::DesyncDetected { .. } => error!("GGRS event: {event:?}"),
                    _ => info!("GGRS event: {event:?}"),
                }
            }
        }
        _ => panic!("This example focuses on p2p."),
    }
}

pub fn print_network_stats_system(
    time: Res<Time>,
    mut timer: ResMut<NetworkStatsTimer>,
    p2p_session: Option<Res<Session<LobbyConfig>>>,
) {
    // print only when timer runs out
    if timer.0.tick(time.delta()).just_finished() {
        if let Some(sess) = p2p_session {
            match sess.as_ref() {
                Session::P2P(s) => {
                    let num_players = s.num_players();
                    for i in 0..num_players {
                        if let Ok(stats) = s.network_stats(i) {
                            println!("NetworkStats for player {}: {:?}", i, stats);
                        }
                    }
                }
                _ => panic!("This examples focuses on p2p."),
            }
        }
    }
}

pub fn checksum_transform(transform: &Transform) -> u64 {
    let mut hasher = FixedState.build_hasher();
    assert!(
        transform.translation.is_finite() && transform.rotation.is_finite(),
        "Hashing is not stable for NaN f32 values."
    );

    transform.translation.x.to_bits().hash(&mut hasher);
    transform.translation.y.to_bits().hash(&mut hasher);
    transform.translation.z.to_bits().hash(&mut hasher);

    transform.rotation.x.to_bits().hash(&mut hasher);
    transform.rotation.y.to_bits().hash(&mut hasher);
    transform.rotation.z.to_bits().hash(&mut hasher);
    transform.rotation.w.to_bits().hash(&mut hasher);

    hasher.finish()
}

pub fn open_session(mut commands: Commands, settings: Res<Settings>) {
    let num_players = settings.players.len();
    assert!(num_players > 0);

    let mut sess_build = SessionBuilder::<LobbyConfig>::new()
        .with_num_players(num_players)
        .with_desync_detection_mode(DesyncDetection::On { interval: 10 })
        .with_max_prediction_window(8)
        .expect("prediction window can't be 0")
        .with_fps(60)
        .expect("invalid fps")
        .with_input_delay(2);
    for (i, player_addr) in settings.players.iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            sess_build = sess_build
                .add_player(PlayerType::Local, i)
                .expect("player handles are invalid or already in use");
        } else {
            // remote players
            let remote_addr = player_addr.parse().expect("invalid address");
            sess_build = sess_build
                .add_player(PlayerType::Remote(remote_addr), i)
                .expect("player handles are invalid or already in use");
        }
    }
    // start the GGRS session
    let socket =
        UdpNonBlockingSocket::bind_to_port(settings.local_port).expect("Couldn't bind to port");
    let sess = sess_build
        .start_p2p_session(socket)
        .expect("Request is invalid");
    println!("starting session");
    commands.insert_resource(Session::P2P(sess));
}

pub fn read_local_inputs(
    mut commands: Commands,
    mut inputs: EventReader<LocalInputEvent>,
    // local_players: Res<LocalPlayers>,
) {
    // let mut local_inputs = HashMap::new();

    for input_ev in inputs.read() {
        // local_inputs.insert(input_ev, LobbyInput(input));
        commands.insert_resource(LocalInputs::<LobbyConfig>(input_ev.0.clone()));
    }
}
