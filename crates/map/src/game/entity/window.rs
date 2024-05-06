use bevy::prelude::*;

use crate::generation::entity::window::WindowConfig;


#[derive(Default, Component)]
pub struct WindowComponent {
    pub config: WindowConfig,
}
