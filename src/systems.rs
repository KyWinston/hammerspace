use crate::components::Interactable;

use bevy::prelude::*;

pub fn check_in_view(cam_q: Query<&Camera>, mut int_q: Query<(&Transform, &mut Interactable)>) {
    for cam in cam_q.iter() {
        if cam.is_active {
            for (_transform, mut interact) in int_q.iter_mut() {
                //todo: check if the interactable is inside the camera frustum
                interact.in_view = true;
            }
        }
    }
}
