mod map_const;
mod context;
mod imp;
mod position;
mod room;

pub mod config;


use bevy::utils::Uuid;
use bevy_ecs_ldtk::ldtk::LdtkJson;

use crate::generation::{imp::get_implementation, room::GeneratedMap};

use self::{config::MapGenerationConfig, context::{AvailableLevel, Connection, MapGenerationContext, Side}, position::Position};

#[derive(Debug, Clone)]
pub struct Room{
    pub level: AvailableLevel, pub position: Position
}

impl Room {
    pub fn create(mut level: AvailableLevel, position: Position) -> Self {

        level.level_iid = Uuid::new_v4().into();
        for c in level.connections.iter_mut() {
            c.level_iid = level.level_iid.clone();
        }

        Self { level: level, position: position }
    }

    pub fn set_connection_to(&mut self, my_connection_index: usize, their_level: &AvailableLevel, their_connection_index: usize) {
        let my_connection = self.level.connections.get_mut(my_connection_index).unwrap();
        my_connection.to = Some(context::ConnectionTo::Room((their_level.clone(), their_connection_index)));
    }

    pub fn is_overlapping(&self, other: &Room) -> bool {

        // find if we are overlapping
        let left_of_other = self.position.0 + self.level.level_size_p.0 < other.position.0;
        let left_of_self = other.position.0 + other.level.level_size_p.0 < self.position.0;

        // Check if one square is above the other
        let above_other = self.position.1 + self.level.level_size_p.1 < other.position.1;
        let above_self = other.position.1 + other.level.level_size_p.1 < self.position.1;

        // If neither square is to the left or above the other, they overlap
        !(left_of_other || left_of_self || above_other || above_self)
    }

    // check if top-left corner is outside or not
    pub fn is_outside(&self, config: &MapGenerationConfig) -> bool {

        let position = &self.position;


        (position.0 > config.max_width || position.0 < (config.max_width * -1)) ||
        (position.1 > config.max_heigth || position.1 < (config.max_heigth * -1))
    }


    pub fn get_connecting_room_position(
        &self, my_connection: &Connection, their_level: &AvailableLevel, their_connection: usize,
        tile_size: &(i32, i32),
    ) -> Position {

        let my_position = &self.position;

        let their_connection = their_level.connections.get(their_connection).unwrap();

        let offset = my_connection.starting_at - their_connection.starting_at;

        // calculate the pixel offset
        match my_connection.side {
            Side::N | Side::S => {
                let offset_pixel = (offset as i32) * tile_size.0;

                Position(
                    my_position.0 + (their_connection.side.get_factor() * (offset_pixel)),
                    my_position.1 + (their_connection.side.get_factor() * -1 * their_level.level_size_p.1),
                )
            },
            Side::W | Side::E => {
                let offset_pixel = (offset as i32) * tile_size.1;

                Position(
                    my_position.0 + (their_connection.side.get_factor() * -1 * their_level.level_size_p.0),
                    my_position.1 + (their_connection.side.get_factor() * (offset_pixel)),
                )
            }
        }


    }
    
}



trait IMapGeneration {
    fn get_spawning_room(&mut self) -> Room; 
    fn get_next_room(&mut self) -> Option<(Room, Connection, Connection)>;
}


pub fn map_generation(mut map_json: LdtkJson, config: config::MapGenerationConfig) -> Result<LdtkJson, ()> {

    let context = MapGenerationContext::from_map(&map_json, config);

    let mut generated_map = GeneratedMap::create(map_json.levels);
    let mut generator = get_implementation(context);

    // select the spawning room
    let room = generator.get_spawning_room();

    generated_map.add_room(&room.level, room.position, None, None);

    while let Some((next_room, next_room_connection, other_room_connection)) = generator.get_next_room() {
        generated_map.add_room(&next_room.level, next_room.position, Some(&next_room_connection), Some(&other_room_connection));
    }

    println!("map generation is over");

    map_json.levels = generated_map.get_generated_levels();

    Ok(map_json)

}
