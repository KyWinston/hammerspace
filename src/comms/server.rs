//! The server side of the example.
//! It is possible (and recommended) to run the server in headless mode (without any rendering plugins).
//!
//! The server will:
//! - spawn a new player entity for each client that connects
//! - read inputs from the clients and move the player entities accordingly
//!
//! Lightyear will handle the replication of entities automatically if you add a `Replicate` component to them.
use std::collections::HashMap;

use bevy::prelude::*;

pub use lightyear::prelude::server::*;
use lightyear::{prelude::*, shared::replication::components::Replicate};

use crate::comms::{
    protocol::{Channel1, Components, Message1, Messages, PlayerBundle},
    shared::shared_movement_behaviour,
};

use super::protocol::{Inputs, PlayerPosition, ServerConnectionManager};

pub struct ExampleServerPlugin;

impl Plugin for ExampleServerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Global {
            client_id_to_entity_id: Default::default(),
        });
        app.add_systems(Startup, init);
        // the physics/FixedUpdates systems that consume inputs should be run in this set
        app.add_systems(FixedUpdate, movement);
        app.add_systems(Update, (send_message, handle_connections));
    }
}

#[derive(Resource)]
pub struct Global {
    pub client_id_to_entity_id: HashMap<ClientId, Entity>,
}

pub fn init(mut commands: Commands, mut connections: ResMut<ServerConnections>) {
    connections.servers[0]
        .start()
        .expect("Failed to start server");
    commands.spawn(
        TextBundle::from_section(
            "Server",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            align_self: AlignSelf::End,
            ..default()
        }),
    );
}

/// Server connection system, create a player upon connection
pub(crate) fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut disconnections: EventReader<DisconnectEvent>,
    mut global: ResMut<Global>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = *connection.context();
        // server and client are running in the same app, no need to replicate to the local client
        let entity = commands.spawn(PlayerBundle::new(
            client_id,
            Vec3::ZERO,
            Replicate {
                prediction_target: NetworkTarget::Single(client_id),
                interpolation_target: NetworkTarget::AllExceptSingle(client_id),
                ..default()
            },
        ));
        // Add a mapping from client id to entity id
        global.client_id_to_entity_id.insert(client_id, entity.id());
        info!("Create entity {:?} for client {:?}", entity.id(), client_id);
    }
    for disconnection in disconnections.read() {
        let client_id = disconnection.context();
        // TODO: handle this automatically in lightyear
        //  - provide a Owned component in lightyear that can specify that an entity is owned by a specific player?
        //  - maybe have the client-id to entity-mapping in the global metadata?
        //  - despawn automatically those entities when the client disconnects
        if let Some(entity) = global.client_id_to_entity_id.remove(client_id) {
            commands.entity(entity).despawn();
        }
    }
}

/// Read client inputs and move players
pub fn movement(
    mut position_query: Query<&mut PlayerPosition>,
    mut input_reader: EventReader<InputEvent<Inputs>>,
    global: Res<Global>,
    tick_manager: Res<TickManager>,
) {
    for input in input_reader.read() {
        let client_id = input.context();
        if let Some(input) = input.input() {
            trace!(
                "Receiving input: {:?} from client: {:?} on tick: {:?}",
                input,
                client_id,
                tick_manager.tick()
            );
            if let Some(player_entity) = global.client_id_to_entity_id.get(client_id) {
                if let Ok(position) = position_query.get_mut(*player_entity) {
                    shared_movement_behaviour(position, input);
                }
            }
        }
    }
}

/// Send messages from server to clients (only in non-headless mode, because otherwise we run with minimal plugins
/// and cannot do input handling)
pub fn send_message(
    mut server: ResMut<ServerConnectionManager>,
    input: Option<Res<ButtonInput<KeyCode>>>,
) {
    if input.is_some_and(|input| input.pressed(KeyCode::KeyM)) {
        let message = Message1(5);
        info!("Send message: {:?}", message);
        server
            .send_message_to_target::<Channel1, Message1>(Message1(5), NetworkTarget::All)
            .unwrap_or_else(|e| {
                error!("Failed to send message: {:?}", e);
            });
    }
}
