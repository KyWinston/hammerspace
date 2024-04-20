//! The client plugin.
//! The client will be responsible for:
//! - connecting to the server at Startup
//! - sending inputs to the server
//! - applying inputs to the locally predicted player (for prediction to work, inputs have to be applied to both the
//! predicted entity and the server entity)
use std::net::{Ipv4Addr, SocketAddr};
use std::str::FromStr;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Duration;
use lightyear::client::prediction::plugin::is_connected;

pub use lightyear::prelude::client::*;
use lightyear::prelude::*;

use crate::comms::protocol::*;
use crate::comms::shared::{shared_config, shared_movement_behaviour};
use crate::comms::{shared, ClientTransports, SharedSettings};

use super::protocol::{Components, Direction, Inputs, Message1, PlayerColor, PlayerPosition};

pub struct ExampleClientPlugin;

impl Plugin for ExampleClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
        app.add_systems(PreUpdate, handle_connection.after(MainSet::Receive));
        // Inputs have to be buffered in the FixedPreUpdate schedule
        app.add_systems(
            FixedPreUpdate,
            buffer_input.in_set(InputSystemSet::BufferInputs),
        );
        app.add_systems(FixedUpdate, player_movement);
        app.add_systems(
            Update,
            (
                receive_message1,
                receive_entity_spawn,
                receive_entity_despawn,
                handle_predicted_spawn,
                handle_interpolated_spawn,
            ),
        );
    }
}

// Startup system for the client
pub(crate) fn init(mut client: ResMut<ClientConnection>) {
    let _ = client.connect();
}

/// Listen for events to know when the client is connected, and spawn a text entity
/// to display the client id
pub(crate) fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<ConnectEvent>,
) {
    for event in connection_event.read() {
        let client_id = event.client_id();
        commands.spawn(TextBundle::from_section(
            format!("Client {}", client_id),
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    }
}

/// System that reads from peripherals and adds inputs to the buffer
/// This system must be run in the
pub(crate) fn buffer_input(
    tick_manager: Res<TickManager>,
    connection: Res<ClientConnection>,
    mut input_manager: ResMut<InputManager<Inputs>>,
    keypress: Res<ButtonInput<KeyCode>>,
) {
    let client_id = connection.id();
    let tick = tick_manager.tick();
    let mut input = Inputs::None;
    let mut direction = Direction {
        up: false,
        down: false,
        left: false,
        right: false,
    };
    if keypress.pressed(KeyCode::KeyW) || keypress.pressed(KeyCode::ArrowUp) {
        direction.up = true;
    }
    if keypress.pressed(KeyCode::KeyS) || keypress.pressed(KeyCode::ArrowDown) {
        direction.down = true;
    }
    if keypress.pressed(KeyCode::KeyA) || keypress.pressed(KeyCode::ArrowLeft) {
        direction.left = true;
    }
    if keypress.pressed(KeyCode::KeyD) || keypress.pressed(KeyCode::ArrowRight) {
        direction.right = true;
    }
    if !direction.is_none() {
        input = Inputs::Direction(direction);
    }
    if keypress.pressed(KeyCode::Backspace) {
        input = Inputs::Delete;
    }
    if keypress.pressed(KeyCode::Space) {
        input = Inputs::Spawn;
    }
    input_manager.add_input(input, tick)
}

/// The client input only gets applied to predicted entities that we own
/// This works because we only predict the user's controlled entity.
/// If we were predicting more entities, we would have to only apply movement to the player owned one.
fn player_movement(
    // TODO: maybe make prediction mode a separate component!!!
    mut position_query: Query<&mut PlayerPosition, With<Predicted>>,
    mut input_reader: EventReader<InputEvent<Inputs>>,

) {
    if <Components as SyncMetadata<PlayerPosition>>::mode() != ComponentSyncMode::Full {
        return;
    }
    for input in input_reader.read() {
        if let Some(input) = input.input() {
            for position in position_query.iter_mut() {
                shared_movement_behaviour(position, input);
            }
        }
    }
}

/// System to receive messages on the client
pub(crate) fn receive_message1(mut reader: EventReader<MessageEvent<Message1>>) {
    for event in reader.read() {
        info!("Received message: {:?}", event.message());
    }
}

/// Example system to handle EntitySpawn events
pub(crate) fn receive_entity_spawn(mut reader: EventReader<EntitySpawnEvent>) {
    for event in reader.read() {
        info!("Received entity spawn: {:?}", event.entity());
    }
}

/// Example system to handle EntitySpawn events
pub(crate) fn receive_entity_despawn(mut reader: EventReader<EntityDespawnEvent>) {
    for event in reader.read() {
        info!("Received entity despawn: {:?}", event.entity());
    }
}

/// When the predicted copy of the client-owned entity is spawned, do stuff
/// - assign it a different saturation
/// - keep track of it in the Global resource
pub(crate) fn handle_predicted_spawn(mut predicted: Query<&mut PlayerColor, Added<Predicted>>) {
    for mut color in predicted.iter_mut() {
        color.0.set_s(0.3);
    }
}

/// When the predicted copy of the client-owned entity is spawned, do stuff
/// - assign it a different saturation
/// - keep track of it in the Global resource
pub(crate) fn handle_interpolated_spawn(
    mut interpolated: Query<&mut PlayerColor, Added<Interpolated>>,
) {
    for mut color in interpolated.iter_mut() {
        color.0.set_s(0.1);
    }
}