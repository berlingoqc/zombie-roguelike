use crate::generation::{context::{MapGenerationContext, MapGenerationData, AvailableLevel, LevelType, Connection}, IMapGeneration, position::Position, Room};

use rand::{Rng, seq::SliceRandom};


struct Map {
    last_generated_room_index: Option<usize>,
    rooms: Vec<Room>,
    // keep track of all room with availabe connections
    rooms_possible: Vec<usize>,
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
            map: Map { last_generated_room_index: None, rooms: vec![], rooms_possible: vec![] }
        }
    }


}

impl BasicMapGeneration {

    fn get_next_room_recursize(&mut self) -> Option<(Room, Connection, Connection)> {
        let room: Option<(Room, Connection, Connection)> = {
            if self.map.last_generated_room_index.is_none() {
                println!("ending generation because last generated room is empty");
                return None;
            }

            let previous_room = self.map.rooms.get_mut(self.map.last_generated_room_index.unwrap()).unwrap();

            let previous_room_c1 = previous_room.clone();

            let mut connections: Vec<&mut crate::generation::context::Connection> = previous_room.level.connections.iter_mut()
                .filter(|i| i.to.is_none())
                .collect();

            let connection_len = connections.len();


            if connection_len == 0 {
                // go to next rooms possible
                if self.map.rooms_possible.len() == 0 {
                    println!("ending generation because list of possible room is empty");
                    return None;
                }

                self.map.last_generated_room_index = self.map.rooms_possible.pop();

                println!("no more connection available for room going to next one");

                return self.get_next_room_recursize();
            }

            println!("room id={} has {} connection available", previous_room_c1.level.level_id, connection_len);

            let connection = connections.iter_mut()
                .skip(self.data.rng.gen_range(0..=connection_len- 1))
                .last()
                .unwrap();


            println!("connection pick {:?}", connection);


            // need to check if i'm out of border first

            // need to check if there is available level for this window
            
            if connection.compatiable_levels.len() == 0 {
                connection.to = Some(crate::generation::context::ConnectionTo::DeadEnd);
                println!("no compatible levels marking as DeadEnd");
                return self.get_next_room_recursize();
            }

            // get a random level of type in the available
            connection.compatiable_levels.shuffle(&mut self.data.rng);

            for level in connection.compatiable_levels
                .iter()
                .filter(|x| x.0.level_type == LevelType::Normal) {


                let level_connection = level.0.connections.get(level.1).unwrap();

                println!("trying level {} with is connection {}", level.0.level_id, level_connection);

                let my_position = previous_room_c1.get_connecting_room_position(&connection, &level.0, level.1, &self.context.tile_size);
                let new_room = Room::create(level.0.clone(), my_position);

                connection.to = Some(crate::generation::context::ConnectionTo::Room(level.clone()));

                return Some((new_room, level_connection.clone(),  connection.clone()));
            }
            

            None
        };


        if let Some(room) = room.as_ref() {
            self.map.rooms.push(room.0.clone());
            self.map.last_generated_room_index = Some(self.map.rooms.len() - 1);
        }

        room

    }
    
}

impl IMapGeneration for BasicMapGeneration {

    fn get_spawning_room(&mut self) -> Room {


        let spawning_levels: Vec<&AvailableLevel> = self.context.available_levels.iter()
            .filter(|i| i.level_type == LevelType::Spawn)
            .collect();

        let spawning_room_def =  spawning_levels.iter()
            .skip(self.data.rng.gen_range(0..=spawning_levels.len() - 1))
            .last();

        if spawning_room_def.is_none() {
            panic!("no spawning room found");
        }

        let spawning_room_def = (*spawning_room_def.unwrap()).clone();

        let x: i32 = self.data.rng.gen_range(self.context.config.get_range_x(spawning_room_def.level_size_p.0));
        let y: i32 = self.data.rng.gen_range(self.context.config.get_range_y(spawning_room_def.level_size_p.1));


        let spawning_room_def = Room::create(spawning_room_def.clone(), Position(x,y));
        self.map.rooms.push(spawning_room_def.clone());
        self.map.last_generated_room_index = Some(0);


        spawning_room_def

    }

    fn get_next_room(&mut self) -> Option<(Room, Connection, Connection)> {
        self.get_next_room_recursize()
    }
}
