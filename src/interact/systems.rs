
use bevy::prelude::*;

use super::components::Interactable;

pub fn check_in_view(
    cam_q: Query<(&GlobalTransform, &Camera)>,
    mut int_q: Query<(&Transform, &mut Interactable)>,
) {
    for (cam_t, cam) in cam_q.iter() {
        if cam.is_active {
            for (transform, mut interact) in int_q.iter_mut() {
                if let Some(ndc) = cam.world_to_ndc(cam_t, transform.translation) {
                    interact.in_view = ndc.x < 1.0
                        && ndc.x > -1.0
                        && ndc.y < 1.0
                        && ndc.y > -1.0
                        && ndc.z < 1.0
                        && ndc.z > 0.0
                }
            }
        }
    }
}
