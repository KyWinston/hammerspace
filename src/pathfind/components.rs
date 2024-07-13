use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use vleue_navigator::{NavMesh, Path};

#[derive(Component)]
pub struct FindingPath(pub Arc<RwLock<(Option<Path>, bool)>>);

#[derive(Component)]
pub struct Navigator {
    pub speed: f32,
}

#[derive(Component)]
pub struct Target {
    pub target: Vec2,
    pub navmesh: Handle<NavMesh>,
}

#[derive(Component)]
pub struct PathNodes {
    pub path: Vec<Vec2>,
}
