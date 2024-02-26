mod map_const;
mod context;
mod imp;
mod position;
mod room;

pub mod config;


use std::rc::Rc;

use bevy_ecs_ldtk::ldtk::LdtkJson;

use crate::generation::{imp::get_implementation, room::GeneratedMap};

use self::{context::{MapGenerationContext, AvailableLevel}, position::Position};


trait IMapGeneration {
    fn get_spawning_room(&mut self) -> (Rc<AvailableLevel>, Position);
    fn get_next_room(&mut self) -> Option<(Rc<AvailableLevel>, Position)>;
}


pub fn map_generation(mut map_json: LdtkJson, config: config::MapGenerationConfig) -> Result<LdtkJson, ()> {

    let context = MapGenerationContext::from_map(&map_json, config);

    let mut generated_map = GeneratedMap::create(map_json.levels);
    let mut generator = get_implementation(context);

    // select the spawning room
    let (starting_room, starting_position) = generator.get_spawning_room();

    generated_map.add_room(&starting_room, starting_position);

    while let Some(next_room) = generator.get_next_room() {
        generated_map.add_room(&next_room.0, next_room.1);
    }

    println!("map generation is over");

    map_json.levels = generated_map.get_generated_levels();

    Ok(map_json)

}
