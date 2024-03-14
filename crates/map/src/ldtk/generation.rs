use std::rc::Rc;

use bevy_ecs_ldtk::ldtk::{FieldValue, LayerInstance, LdtkJson, Level, NeighbourLevel};

use crate::generation::{config::MapGenerationConfig, context::{populate_level_connections, scan_height_side, scan_width_side, AvailableLevel, LevelType, MapGenerationContext, MapGenerationData, Side}, map_const, room::{Room, RoomConnection}, IMapGenerator};

fn get_level_field(level: &Level, name: &str) -> Option<FieldValue> {
    level.field_instances.iter()
        .find(|instance| name == instance.identifier)
        .map(|instance| instance.value.clone())
}


fn to_available_level(level: &Level, tile_size: &(i32, i32)) -> AvailableLevel {
    let level_size: (usize, usize) = (
        (level.px_wid / tile_size.0) as usize,
        (level.px_hei / tile_size.1) as usize
    );

    // identify each level connection
    let connection_layer: &LayerInstance = level.layer_instances.as_ref().map(|layer_instance| {
        layer_instance.into_iter()
            .find(|item| map_const::LAYER_CONNECTION == item.identifier)
            .ok_or_else(|| "Failed to find LevelConnetion Layer on level")
    }).unwrap_or_else(|| Err("No Layers present")).unwrap();

    let grid: Vec<&[i32]> = connection_layer.int_grid_csv.chunks(level_size.0 as usize).collect();

    let level_type = {
        let is_spawn = get_level_field(&level, map_const::LEVEL_FIELD_SPAWN).map_or(false, |x| {
            if let FieldValue::Bool(value) =x {
                value
            } else {
                false
            }
        });

        if is_spawn == true {
            LevelType::Spawn
        } else {
            LevelType::Normal
        }
    };


    let mut available_level = AvailableLevel { 
        level_id: level.identifier.clone(),
        connections: vec![],
        level_size,
        level_size_p: (level_size.0 as i32 * tile_size.0, level_size.1 as i32 * tile_size.1),
        level_type,
    };


    let mut connections= vec![];
    let mut index = 0;
    scan_width_side(&mut connections, &mut index, &available_level, &level_size, &grid, 0, Side::N);
    scan_width_side(&mut connections, &mut index, &available_level, &level_size, &grid, level_size.1 - 1, Side::S);

    scan_height_side(&mut connections, &mut index, &available_level, &level_size, &grid, 0, Side::W);
    scan_height_side(&mut connections, &mut index, &available_level, &level_size, &grid, level_size.0 - 1, Side::E);

    available_level.connections = connections;

    available_level
}


pub fn from_map(map_json: &LdtkJson, config: MapGenerationConfig) -> MapGenerationContext {
    if map_json.levels.len() < 1 {
        eprintln!("to few level present in the project");
    }

    let tile_size = (map_json.default_entity_width, map_json.default_entity_height);

    let first_level = map_json.levels.get(0).unwrap();

    let level_size = (
        first_level.px_wid / tile_size.0,
        first_level.px_hei / tile_size.1
    );

    println!("starting level generation with config \nseed={} \ntilse_size={}x{} \nlevel_size={}x{}\nmap_size={}x{}", 
        config.seed, tile_size.0, tile_size.1, level_size.0, level_size.1,
        config.max_width, config.max_heigth
    );

    let mut available_levels: Vec<AvailableLevel> = map_json.levels.iter()
        .map(|item| {
            to_available_level(&item, &tile_size)
        }).collect();


    populate_level_connections(&mut available_levels);


    let available_levels = available_levels.iter()
        .map(|x| Rc::new(x.clone()))
        .collect();

    MapGenerationContext {
        level_size,
        tile_size,
        config,
        available_levels,
    }
}




#[derive(Debug, Clone)]
pub struct GeneratedRoom {
    level: Level,
}


impl GeneratedRoom {

    pub fn create(original_ldtk_levels: &Vec<Level>, room: &Room) -> Self {
        let mut level = original_ldtk_levels.iter()
            .find(|item| item.identifier == room.level_def.level_id)
            .expect("failed to find level from original")
            .clone();

        level.iid = room.level_iid.clone();
        level.identifier = room.level_iid.clone();

        GeneratedRoom{
            level
        }
    }
    
}



pub struct GeneratedMap {
    ldtk_json: LdtkJson,
    generated_rooms: Vec<GeneratedRoom>,
}

impl GeneratedMap {
    pub fn create(ldtk_json: LdtkJson) -> Self {
        GeneratedMap {
            ldtk_json,
            generated_rooms: vec![],
        }
    }


    pub fn get_generated_map(&self) -> LdtkJson {
        let mut new_map = self.ldtk_json.clone();

        new_map.levels = self.generated_rooms.iter().enumerate().map(|(i,x)| {
            let mut r = x.level.clone();
            r.identifier = format!("Level_{}", i);
            // generate new iid for all subressources maybe ????
            r
        }).collect();


        new_map
    }
}

impl IMapGenerator for GeneratedMap {
    fn add_room(&mut self, room: &Room, connection_used: Option<&RoomConnection>, connected_to: Option<&RoomConnection>) {
        let mut generated_room = GeneratedRoom::create(&self.ldtk_json.levels, room);

        println!("adding room id={} type={:?} from_level={} position={}", room.level_iid, room.level_def.level_type, room.level_def.level_id, room.position);

        generated_room.level.world_x = room.position.0;
        generated_room.level.world_y = room.position.1;
        generated_room.level.neighbours.clear();

        if let Some(connected_to) = connected_to {


            let connection_used = connection_used.unwrap();

            generated_room.level.neighbours.push(NeighbourLevel{
                level_iid: connected_to.level_iid.clone(),
                dir: connection_used.side.to_dir_str().into(),
                ..Default::default()
            });

            // find the other room and me as it's neighbours
            let linked_room = self.generated_rooms.iter_mut()
                .find(|r| r.level.iid == connected_to.level_iid)
                .unwrap();


            println!("  connecting my side={:?} index={} with side={:?} index={} of room id={} from_level={} position={}x{}",
               connection_used.side, connection_used.index, connected_to.side, connected_to.index, connected_to.level_iid,
               connected_to.level_id, linked_room.level.world_x, linked_room.level.world_y,
            );
            
            linked_room.level.neighbours.push(NeighbourLevel { 
                dir: connected_to.side.to_dir_str().into(),
                level_iid: room.level_iid.clone(),
                ..Default::default()
            })

        }

        println!("");

        self.generated_rooms.push(generated_room);

    }
}
