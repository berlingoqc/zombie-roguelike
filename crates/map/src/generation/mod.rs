mod map_const;
mod context;
mod imp;
mod position;
mod room;

pub mod config;


use bevy_ecs_ldtk::ldtk::LdtkJson;

use crate::generation::{imp::get_implementation, room::GeneratedMap};

use self::{context::{MapGenerationContext, AvailableLevel, Connection, Side}, position::Position};

#[derive(Debug, Clone)]
pub struct Room(pub AvailableLevel, pub Position);

impl Room {


    pub fn is_overlapping(&self, other: &Room) -> bool {

        // find if we are overlapping
        let left_of_other = self.1.0 + self.0.level_size_p.0 < other.1.0;
        let left_of_self = other.1.0 + other.0.level_size_p.0 < self.1.0;

        // Check if one square is above the other
        let above_other = self.1.1 + self.0.level_size_p.1 < other.1.1;
        let above_self = other.1.1 + other.0.level_size_p.1 < self.1.1;

        // If neither square is to the left or above the other, they overlap
        !(left_of_other || left_of_self || above_other || above_self)
    }

    pub fn get_connecting_room_position(
        &self, my_connection: &Connection, their_level: &AvailableLevel, their_connection: usize,
        tile_size: &(i32, i32),
    ) -> Position {

        let my_position = &self.1;

        let their_connection = their_level.connections.get(their_connection).unwrap();

        let offset = my_connection.starting_at - their_connection.starting_at;

        // calculate the pixel offset
        match my_connection.side {
            Side::N | Side::S => {
                let offset_pixel = (offset as i32) * tile_size.0;

                Position(
                    my_position.0 + (my_connection.side.get_factor() * (their_level.level_size_p.0 + offset_pixel)),
                    my_position.1 + (my_connection.side.get_factor() * their_level.level_size_p.1),
                )
            },
            Side::W | Side::E => {
                let offset_pixel = (offset as i32) * tile_size.1;

                Position(
                    my_position.0 + (my_connection.side.get_factor() * their_level.level_size_p.0),
                    my_position.1 + (my_connection.side.get_factor() * (their_level.level_size_p.1 + offset_pixel)),
                )
            }
        }


    }
    
}



trait IMapGeneration {
    fn get_spawning_room(&mut self) -> Room; 
    fn get_next_room(&mut self) -> Option<Room>;
}


pub fn map_generation(mut map_json: LdtkJson, config: config::MapGenerationConfig) -> Result<LdtkJson, ()> {

    let context = MapGenerationContext::from_map(&map_json, config);

    let mut generated_map = GeneratedMap::create(map_json.levels);
    let mut generator = get_implementation(context);

    // select the spawning room
    let room = generator.get_spawning_room();

    generated_map.add_room(&room.0, room.1);

    while let Some(next_room) = generator.get_next_room() {
        generated_map.add_room(&next_room.0, next_room.1);
    }

    println!("map generation is over");

    map_json.levels = generated_map.get_generated_levels();

    Ok(map_json)

}
