use bevy::utils::Uuid;
use bevy_ecs_ldtk::ldtk::Level;

use super::{context::AvailableLevel, position::Position};

#[derive(Debug, Clone)]
pub struct GeneratedRoom {
    level: Level,
}


impl GeneratedRoom {

    pub fn create(original_ldtk_levels: &Vec<Level>, available_level: &AvailableLevel) -> Self {
        let mut level = original_ldtk_levels.iter()
            .find(|item| item.identifier == available_level.level_id)
            .expect("failed to find level from original")
            .clone();

        level.iid = Uuid::new_v4().to_string();
        level.identifier = level.iid.clone();

        GeneratedRoom{
            level
        }
    }
    
}

pub struct GeneratedMap {
    original_ldtk_levels: Vec<Level>,
    generated_rooms: Vec<GeneratedRoom>,
}

impl  GeneratedMap {
    pub fn create(original_ldtk_levels: Vec<Level>) -> Self {
        GeneratedMap {
            original_ldtk_levels,
            generated_rooms: vec![],
        }
    }


    pub fn get_generated_levels(&self) -> Vec<Level> {
        self.generated_rooms.iter().map(|x| x.level.clone()).collect()
    }


    pub fn add_room(&mut self, level: &AvailableLevel, position: Position) {
        let mut room = GeneratedRoom::create(&self.original_ldtk_levels, level);

        println!("adding room type={:?} id={} position={}", level.level_type, level.level_id, position);

        room.level.world_x = position.0;
        room.level.world_y = position.1;

        self.generated_rooms.push(room);
    }

}
