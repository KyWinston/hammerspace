use bevy::{
    input::gamepad::{ButtonSettings, GamepadConnection, GamepadEvent, GamepadSettings},
    prelude::*,
};

use super::resources::Controller;

pub fn configure_gamepad(
    my_gamepad: Option<Res<Controller>>,
    mut settings: ResMut<GamepadSettings>,
) {
    let _gamepad = if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        gp.0
    } else {
        // no gamepad is connected
        println!("gamepad not found");
        return;
    };

    // for buttons (or axes treated as buttons):
    let mut button_settings = ButtonSettings::default();
    // require them to be pressed almost all the way, to count
    button_settings.set_press_threshold(0.5);
    // require them to be released almost all the way, to count
    button_settings.set_release_threshold(0.1);

    settings.default_button_settings = button_settings;
}

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<Controller>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for ev in gamepad_evr.read() {
        match &ev {
            GamepadEvent::Connection(gp_ev) => {
                let id = gp_ev.gamepad;
                match &gp_ev.connection {
                    GamepadConnection::Connected(info) => {
                        println!(
                            "New gamepad connected with ID: {:?}, name: {}",
                            id, info.name
                        );
                        if my_gamepad.is_none() {
                            commands.insert_resource(Controller(id));
                        }
                    }
                    GamepadConnection::Disconnected => {
                        println!("Lost gamepad connection with ID: {:?}", id);

                        if let Some(Controller(old_id)) = my_gamepad.as_deref() {
                            if *old_id == id {
                                commands.remove_resource::<Controller>();
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
