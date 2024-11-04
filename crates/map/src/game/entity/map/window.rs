use bevy::prelude::*;

use crate::generation::entity::window::WindowConfig;

#[derive(Default, Component, Reflect)]
pub struct WindowComponent {
    pub config: WindowConfig,
}
