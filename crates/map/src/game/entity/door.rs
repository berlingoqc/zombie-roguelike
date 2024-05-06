use bevy::prelude::*;

use crate::generation::entity::door::DoorConfig;

#[derive(Default, Component, Reflect)]
pub struct DoorComponent {
    pub config: DoorConfig,
}
