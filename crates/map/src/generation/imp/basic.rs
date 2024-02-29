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

    // I NEED TO MARK BOTH CONNECTION AS CLOSE NOT JUST THE ONE

    fn get_next_room_recursize(&mut self) -> Option<(Room, Connection, Connection)> {
        if self.map.rooms.len() > 4 {
            return None;
        }
        let room: Option<(Room, Connection, Connection)> = {
            if self.map.last_generated_room_index.is_none() {
                return None;
            }

            let previous_room_index = self.map.last_generated_room_index.unwrap();

            let previous_room = self.map.rooms.get_mut(previous_room_index).unwrap();

            let previous_room_c1 = previous_room.clone();

            let mut connections: Vec<&mut crate::generation::context::Connection> = previous_room.level.connections.iter_mut()
                .filter(|i| i.to.is_none())
                .collect();

            let connection_len = connections.len();

            if connection_len == 0 {
                if let Some(index) = self.map.rooms_possible.iter().position(|&x| x == previous_room_index) {
                    self.map.rooms_possible.remove(index);
                }

                if self.map.rooms_possible.len() == 0 {
                    return None;
                }

                self.map.last_generated_room_index = self.map.rooms_possible.get(self.data.rng.gen_range(0..=(self.map.rooms_possible.len() - 1))).copied();

                self.get_next_room_recursize()
            } else {
                let connection = connections.iter_mut()
                    .skip(self.data.rng.gen_range(0..=connection_len- 1))
                    .last()
                    .unwrap();


                if connection.compatiable_levels.len() == 0 {
                    connection.to = Some(crate::generation::context::ConnectionTo::DeadEnd);
                    println!("no compatible levels marking as DeadEnd");
                    self.get_next_room_recursize()
                } else {
                    // get a random level of type in the available
                    connection.compatiable_levels.shuffle(&mut self.data.rng);

                    let compatible_level = connection.compatiable_levels
                        .iter()
                        .last()
                        .unwrap();

                    let compatible_level_def = self.context.available_levels.iter()
                        .find(|l| l.level_id == compatible_level.0)
                        .unwrap();

                    let level_connection = compatible_level_def.connections.get( compatible_level.1).unwrap();

                    let my_position = previous_room_c1.get_connecting_room_position(&connection, &compatible_level_def, compatible_level.1, &self.context.tile_size);
                    let mut new_room = Room::create(compatible_level_def.clone(), my_position);

                    if new_room.is_outside(&self.context.config) {
                        connection.to = Some(crate::generation::context::ConnectionTo::OutSide);

                        self.get_next_room_recursize()

                    } else {
                        connection.to = Some(crate::generation::context::ConnectionTo::Room((compatible_level_def.clone(), compatible_level.1)));
                        new_room.set_connection_to(level_connection.index, &previous_room_c1.level, connection.index);

                        Some((new_room, level_connection.clone(),  connection.clone()))
                    }

                }
            }
        };


        if let Some(room) = room.as_ref() {
            self.map.rooms.push(room.0.clone());
            let index = self.map.rooms.len() - 1;
            self.map.rooms_possible.push(index);
            // if we switch index , if the current index one still has connection should be in rooms_possible if not present already
            // self.map.last_generated_room_index = Some(index);
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
