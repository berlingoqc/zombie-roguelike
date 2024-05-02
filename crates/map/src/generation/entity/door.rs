use crate::generation::{position::Position, room::ConnectionTo};


#[derive(Debug, Clone)]
pub struct DoorConfig {
    pub connection: ConnectionTo,
    pub level_iid: String,
    pub position: Position,
    pub size: (i32, i32),

    // cost to open the door
    pub cost: i32,
    // if the door need electricity to be open
    pub electrify: bool,
}