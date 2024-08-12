use bevy::{prelude::*, scene::ron::from_str};
use bevy_ggrs::{GgrsApp, GgrsPlugin, GgrsSchedule, ReadInputs};
use components::PlayerPosition;
use events::ControllerLoadEvent;
use resources::LobbyConfig;
use systems::{checksum_transform, read_local_inputs};

use crate::{
    assembler::AssetLoadState,
    network::{resources::Settings, systems::print_events_system},
};

use self::{
    events::connect_to_players,
    resources::NetworkStatsTimer,
    systems::{open_session, print_network_stats_system, start_match_session},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let settings_str = include_str!("settings.ron");
        let settings = from_str::<Settings>(settings_str).unwrap();

        app.add_event::<ControllerLoadEvent>()
            .insert_resource::<Settings>(settings)
            .init_state::<NetState>()
            .add_plugins(GgrsPlugin::<LobbyConfig>::default())
            .add_systems(
                OnEnter(AssetLoadState::Loaded),
                (open_session, connect_to_players.after(open_session)),
            )
            .add_systems(
                GgrsSchedule,
                (
                    print_events_system.before(print_network_stats_system),
                    print_network_stats_system.before(start_match_session),
                    start_match_session.after(connect_to_players),
                )
                    .run_if(in_state(AssetLoadState::Loading)),
            )
            .add_systems(ReadInputs, read_local_inputs)
            .insert_resource(NetworkStatsTimer(Timer::from_seconds(
                2.0,
                TimerMode::Repeating,
            )))
            .set_rollback_schedule_fps(60)
            .checksum_component::<Transform>(checksum_transform)
            .rollback_component_with_clone::<PlayerPosition>();
    }
}

#[derive(States, Debug, Default, Hash, Eq, PartialEq, Clone)]
pub enum NetState {
    #[default]
    Lobby,
    Offline,
    Connected,
}

// #[cfg(test)]
// mod test {
//     use crate::{
//         game::controller::events::ControllerLoadEvent, network::events::connect_to_players,
//     };
//     use bevy::{
//         color::{palettes::css::BLUE, Color, ColorToComponents, Srgba},
//         prelude::World,
//     };
//     #[test]
//     pub fn can_run_offline() {
//         // Setup app
//         let mut world: World = World::default();
//         world.send_event(ControllerLoadEvent {
//             player_color: Color::srgb_from_array(BLUE.to_f32_array_no_alpha()),
//             player_number: 0,
//         });
//         let sys_id = world.register_system(connect_to_players);
//         world.run_system(sys_id).expect("system cannot be run");
//         println!("{:?}", world.components());
//         assert!(!world.components().is_empty());
//     }
// }
