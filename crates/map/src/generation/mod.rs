mod imp;
mod position;

pub mod room;
pub mod map_const;
pub mod config;
pub mod context;


use crate::generation::imp::get_implementation;

use self::{context::MapGenerationContext, room::{Room, RoomConnection}};



trait IMapGeneration {
    fn get_spawning_room(&mut self) -> Room; 
    fn get_next_room(&mut self) -> Option<(Room, RoomConnection, RoomConnection)>;
}

pub trait IMapGenerator {
    fn add_room(&mut self, room: &Room, connection_used: Option<&RoomConnection>, connected_to: Option<&RoomConnection>);

}


pub fn map_generation(mut context: MapGenerationContext, map_generator: &mut impl IMapGenerator) -> Result<(), ()> {


    //let mut generated_map = GeneratedMap::create(map_json.levels);
    let mut generator = get_implementation(context);

    // select the spawning room
    let room = generator.get_spawning_room();

    map_generator.add_room(&room, None, None);

    while let Some((next_room, next_room_connection, other_room_connection)) = generator.get_next_room() {
        map_generator.add_room(&next_room, Some(&next_room_connection), Some(&other_room_connection));
    }

    Ok(())

    /*
    map_json.levels = generated_map.get_generated_levels();

    Ok(map_json)
    */

}
