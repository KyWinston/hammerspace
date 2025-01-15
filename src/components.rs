use bevy::prelude::*;

///actors are able to be interacted with and are in turn able to interact with the player
///this includes hostile npcs that can attack the player
#[derive(Component)]
#[require(Interactable)]
pub struct Actor;

///use this if interacting with something would start a dialogue sequence
/// actors with dialogue can react to the players location relative to them (this can be used to have the actor look at the player)
/// entities with dialogue for the player cannot deal damage.
#[derive(Component)]
#[require(Interactable)]
pub struct HasDialogue(pub String);

///interactables are able to be interacted with by the player, either by locking onto them or by talking to them
///this includes hostile npcs that can attack the player
#[derive(Component, Default)]
pub struct Interactable(pub String);
