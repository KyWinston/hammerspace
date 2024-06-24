use std::sync::{Arc, RwLock};

use bevy::prelude::*;
use vleue_navigator::{NavMesh, Path};

#[derive(Component)]
pub struct FindingPath(pub Arc<RwLock<(Option<Path>, bool)>>);

#[derive(Component)]
struct Navigator {
    speed: f32,
}

#[derive(Component)]
pub struct Target {
    target: Vec2,
    navmesh: Handle<NavMesh>,
}

#[derive(Component)]
struct PathNodes {
    path: Vec<Vec2>,
}
