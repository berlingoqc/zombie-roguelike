use std::rc::Rc;

use crate::generation::{context::{MapGenerationContext, MapGenerationData, AvailableLevel}, IMapGeneration, position::Position};

use rand::Rng;


type Room = (AvailableLevel, Position);

struct Map {
    last_generated_room: Option<Room>,
    rooms: Vec<Room>,

    // keep track of all room with availabe connections
    rooms_possible: Vec<Room>,
}


pub struct BasicMapGeneration {
    context: MapGenerationContext,
    data: MapGenerationData,

    map: Map
}

impl BasicMapGeneration {

    pub fn create(context: MapGenerationContext) -> Self {
        BasicMapGeneration {
            data: MapGenerationData::from_context(&context),
            context,
            map: Map { last_generated_room: None, rooms: vec![], rooms_possible: vec![] }
        }
    }


}

impl IMapGeneration for BasicMapGeneration {

    fn get_spawning_room(&mut self) ->  (AvailableLevel, Position) {

        let spawning_room_def = self.context.available_levels.spawning.values()
            .skip(self.data.rng.gen_range(0..=self.context.available_levels.spawning.len() - 1))
            .last();

        if spawning_room_def.is_none() {
            panic!("no spawning room found");
        }

        let spawning_room_def = spawning_room_def.unwrap();

        let x: i32 = self.data.rng.gen_range(self.context.config.get_range_x(spawning_room_def.level_size_p.0));
        let y: i32 = self.data.rng.gen_range(self.context.config.get_range_y(spawning_room_def.level_size_p.1));


        let spawning_room_def = (spawning_room_def.clone(), Position(x,y));
        self.map.rooms_possible.push(spawning_room_def.clone());

        spawning_room_def

    }

    fn get_next_room(&mut self) -> Option<(AvailableLevel, Position)> {

        if self.map.rooms_possible.len() == 0 {
            return None;
        }

        None
    }
}
