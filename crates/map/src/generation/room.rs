use bevy::utils::Uuid;
use bevy_ecs_ldtk::ldtk::{Level, NeighbourLevel};

use super::{context::{AvailableLevel, Connection}, position::Position};

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

        level.iid = available_level.level_iid.clone();
        level.identifier = available_level.level_iid.clone();
        //format!("{}:{}", level.identifier, level.iid.clone());

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
        self.generated_rooms.iter().enumerate().map(|(i,x)| {
            let mut r = x.level.clone();
            r.identifier = format!("Level_{}", i);
            r
        }).collect()
    }


    pub fn add_room(&mut self, level: &AvailableLevel, position: Position, connection_used: Option<&Connection>, connected_to: Option<&Connection>) {
        let mut room = GeneratedRoom::create(&self.original_ldtk_levels, level);

        println!("adding room type={:?} id={} position={}", level.level_type, level.level_id, position);

        room.level.world_x = position.0;
        room.level.world_y = position.1;
        room.level.neighbours.clear();
        if let Some(connected_to) = connected_to {
            room.level.neighbours.push(NeighbourLevel{
                level_iid: connected_to.level_id.clone(),
                dir: connection_used.unwrap().side.to_dir_str().into(),
                ..Default::default()
            });

            // find the other room and me as it's neighbours
            let linked_room = self.generated_rooms.iter_mut()
                .find(|r| r.level.iid == connected_to.level_iid)
                .unwrap();
            
            linked_room.level.neighbours.push(NeighbourLevel { 
                dir: connected_to.side.to_dir_str().into(),
                level_iid: room.level.iid.clone(),
                ..Default::default()
            })

        }
        self.generated_rooms.push(room);

    }

}
