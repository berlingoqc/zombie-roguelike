use bevy::prelude::*;

#[derive(Debug, Clone, Default, Reflect)]
pub struct RoomConfig {
    pub spawn: bool,
}
