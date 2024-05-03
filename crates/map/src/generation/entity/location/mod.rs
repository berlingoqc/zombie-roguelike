use crate::generation::position::Position;

#[derive(Debug, Clone)]
pub struct EntityLocation {
    pub level_iid: String,
    pub position: Position,
    pub size: (i32, i32),
}

#[derive(Debug, Clone)]
pub struct EntityLocations {
    pub doors: Vec<EntityLocation>,
    pub sodas: Vec<EntityLocation>,
    pub player_spawns: Vec<EntityLocation>,
    pub zombie_spawns: Vec<EntityLocation>,
    pub crates: Vec<EntityLocation>,
    pub weapons: Vec<EntityLocation>,
    pub windows: Vec<EntityLocation>,
}