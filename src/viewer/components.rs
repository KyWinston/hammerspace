use bevy::prelude::*;

#[derive(Component)]
pub struct Setpiece;


#[derive(Component)]
pub struct PanOrbitCamera {
    pub orbit: f32,
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub panning: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            orbit: 0.0,
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
            panning: false,
        }
    }
}
