use std::ops::{Add, Mul};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct PlayerPosition(pub Vec3);

impl Mul<f32> for &PlayerPosition {
    type Output = PlayerPosition;

    fn mul(self, rhs: f32) -> Self::Output {
        PlayerPosition(self.0 * rhs)
    }
}

impl Add<PlayerPosition> for PlayerPosition {
    type Output = PlayerPosition;
    fn add(self, rhs: PlayerPosition) -> Self::Output {
        PlayerPosition(self.0 + rhs.0)
    }
}