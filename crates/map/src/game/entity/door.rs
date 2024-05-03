use bevy::prelude::*;

use crate::generation::entity::door::DoorConfig;

#[derive(Default, Component)]
pub struct DoorComponent {
    pub config: DoorConfig,
}
