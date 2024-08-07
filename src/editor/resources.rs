use bevy::prelude::*;

use super::components::Page;

#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct CurrentPage(pub Page);


