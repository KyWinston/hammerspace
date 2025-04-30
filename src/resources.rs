use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct HammerspaceConfig {
    pub level_folder: String,
}

impl Default for HammerspaceConfig {
    fn default() -> Self {
        Self {
            level_folder: "".to_string(),
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
