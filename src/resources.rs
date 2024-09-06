use bevy::prelude::*;

#[derive(Resource,Clone)]
pub struct HammerspaceConfig {
    pub level_folder: String,
    pub lights_identifier: String,
    pub collision_identifier: String,
    pub spawn_identifier: String,
}

impl Default for HammerspaceConfig {
    fn default() -> Self {
        Self {
            level_folder: "".to_string(),
            lights_identifier: "_light".to_string(),
            collision_identifier: "_colllider".to_string(),
            spawn_identifier: "_spawn".to_string(),
        }
    }
}

impl HammerspaceConfig {
    pub fn new(level_folder: String) -> Self {
        Self {
            level_folder,
            ..default()
        }
    }
}
