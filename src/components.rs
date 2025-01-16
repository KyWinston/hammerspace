use bevy::prelude::*;

/// actors are able to be interacted with and are in turn able to interact with the player
/// this includes hostile npcs that can attack the player
#[derive(Component, Default)]
#[require(Interactable)]
pub struct Actor;

impl Actor {
    pub fn list_valid_interacts(
        location: Vec3,
        distance: f32,
        int_q: Query<(&Transform, &Interactable), Without<Player>>,
        exclude_offscreen: bool,
    ) -> Vec<Transform> {
        let mut list: Vec<Transform> = vec![];
        for (transform, int) in int_q.iter() {
            let range = location.distance(transform.translation);
            if range < distance && int.in_range(range, exclude_offscreen) {
                list.push(*transform);
            }
        }
        list.sort_by(|a, b| a.translation.length().total_cmp(&b.translation.length()));
        list
    }
}

#[derive(Component, Default)]
#[require(Actor)]
pub struct Player;

/// use this if interacting with something would start a dialogue sequence
/// actors with dialogue can react to the players location relative to them (this can be used to have the actor look at the player)
/// entities with dialogue for the player cannot deal damage.
#[derive(Component, Default)]
#[require(Interactable)]
pub struct HasDialogue;

/// interactables are able to be interacted with by the player, either by locking onto them or by talking to them
/// this includes hostile npcs that can attack the player
#[derive(Component, Default)]
pub struct Interactable {
    pub tag: String,
    interact_distace: f32,
    pub(crate) in_view: bool,
}

impl Interactable {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            interact_distace: 50.0,
            in_view: false,
        }
    }
    pub fn in_range(&self, distance: f32, exclude_offscreen: bool) -> bool {
        !exclude_offscreen || self.in_view && distance < self.interact_distace
    }
}
