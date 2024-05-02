use crate::generation::position::Position;

#[derive(Debug, Clone)]
pub struct MultiTileEntityLocation {
    pub position: Position,
    pub size: (i32, i32),
}

#[derive(Debug, Clone)]
pub struct EntityLocation {
    pub position: Position,
}

#[derive(Debug, Clone)]
pub struct EntityLocations {
    pub doors: Vec<MultiTileEntityLocation>,
    pub sodas: Vec<EntityLocation>,
    pub player_spawns: Vec<EntityLocation>,
    pub zombie_spawns: Vec<EntityLocation>,
    pub weapon_crates: Vec<EntityLocation>,
    pub weapons: Vec<EntityLocation>,
    pub windows: Vec<MultiTileEntityLocation>,
}

impl EntityLocations {
    pub fn to_world_position(&self, room_position: Position) -> EntityLocations {
        EntityLocations {
            doors: vec![],
            sodas: vec![],
            player_spawns: vec![],
            zombie_spawns: vec![],
            weapon_crates: vec![],
            weapons: vec![],
            windows: vec![],
        }
    }
}
