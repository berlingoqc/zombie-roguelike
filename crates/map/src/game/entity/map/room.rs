use bevy::prelude::*;

use crate::generation::entity::room::RoomConfig;

#[derive(Default, Component, Reflect)]
pub struct RoomComponent {
    pub config: RoomConfig,
}
