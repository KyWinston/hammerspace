use bevy::prelude::*;

pub mod components;
pub mod systems;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {}
}
