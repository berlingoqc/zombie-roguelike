use bevy::prelude::*;

#[derive(Debug, Clone, Default, Reflect)]
pub struct DoorConfig {
    // cost to open the door
    pub cost: i32,
    // if the door need electricity to be open
    pub electrify: bool,
}
