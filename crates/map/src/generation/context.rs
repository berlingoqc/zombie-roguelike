

use bevy_ecs_ldtk::{ldtk::{LdtkJson, Level}, prelude::{LayerInstance, FieldValue}};

use super::{map_const, config::MapGenerationConfig};

use std::{collections::HashMap, usize, rc::Rc, fmt::Display};


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
   N,
   S,
   W,
   E,
}

impl Side {

    pub fn get_opposite(&self) -> Self {
        match self {
            Side::N => Side::S,
            Side::S => Side::N,
            Side::W => Side::E,
            Side::E => Side::W
        }
    }

    pub fn get_factor(&self) -> i32 {
        match self {
            Side::N | Side::W => -1,
            Side::S | Side::E => 1,
            
        }
    }

    pub fn is_opposite(&self, other: Side) -> bool {
        other == self.get_opposite()
    }
    
}


#[derive(Clone, Debug, PartialEq)]
pub enum LevelType {
    Spawn,
    Normal,
}


#[derive(Debug, Clone)]
pub enum ConnectionTo {
    Room((AvailableLevel, usize)),
    DeadEnd,
    OutSide,
}
 

#[derive(Debug, Clone)]
pub struct Connection {
    pub index: usize,
    pub size: usize,
    pub side: Side,
    pub starting_at: usize,
    
    pub level_id: String,
    pub compatiable_levels: Vec<(AvailableLevel, usize)>,

    pub to: Option<ConnectionTo>,
}

impl Display for Connection {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "side={:?} starting_at={} size={}", self.size, self.starting_at, self.size)
    }
}

impl Connection {

    fn are_matching(&self, other: &Connection) -> bool {
        self.side.is_opposite(other.side) &&
        self.starting_at == other.starting_at &&
        self.size == other.size
    }
}



#[derive(Debug, Clone)]
pub struct AvailableLevel {
    // identifier of the original level
    pub level_id: String,
    // level size in tile
    pub level_size: (usize, usize),
    // level size in px
    pub level_size_p: (i32, i32),
    // type of level
    pub level_type: LevelType,

    pub connections: Vec<Connection>,
}

pub type AvailableLevels = Vec<AvailableLevel>;

#[derive(Debug)]
pub struct AvailableLevelsPerType {
    pub spawning: AvailableLevels,
    pub regular: AvailableLevels,
}



fn scan_width_side(
    connections: &mut Vec<Connection>,
    level: &AvailableLevel,
    level_size: &(usize, usize),
    grid: &Vec<&[i32]>,
    row_index: usize,
    side: Side,
) {
    let mut i = 0;
    while i < level_size.0 {
        if grid[row_index][i] == 1 {
            let mut size = 1;
            while size + i < level_size.0 {
                if grid[row_index][size + i] == 0 {
                    break;
                }
                size += 1;
            }

            connections.push(Connection { 
                index: level.connections.len(),
                size,
                side: side.clone(),
                starting_at: i,
                level_id: level.level_id.clone(),
                to: None,
                compatiable_levels: vec![],
            });

            i += size;
        } else {
            i += 1;
        }
    }
}

fn scan_height_side(
    connections: &mut Vec<Connection>,
    level: &AvailableLevel,
    level_size: &(usize, usize),
    grid: &Vec<&[i32]>,
    column_index: usize,
    side: Side,
) {
    let mut i = 0;
    while i < level_size.1 {
        if grid[i][column_index] == 1 {
            let mut size = 1;
            while size + i < level_size.1 {
                if grid[size + i][column_index] == 0 {
                    break;
                }
                size += 1;
            }

            connections.push(Connection { 
                index: connections.len(),
                size,
                side: side.clone(),
                starting_at: i,
                level_id: level.level_id.clone(),
                to: None,
                compatiable_levels: vec![],
            });

            i += size;
        } else {
            i += 1;
        }
    }

}

fn get_level_field(level: &Level, name: &str) -> Option<FieldValue> {
    level.field_instances.iter()
        .find(|instance| name == instance.identifier)
        .map(|instance| instance.value.clone())
}


impl AvailableLevel {

    fn from_level(level: &Level, tile_size: &(i32, i32)) -> AvailableLevel {
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
 

        let mut available_level = Self { 
            level_id: level.identifier.clone(),
            connections: vec![],
            level_size,
            level_size_p: (level_size.0 as i32 * tile_size.0, level_size.1 as i32 * tile_size.1),
            level_type,
        };


        let mut connections= vec![];
        scan_width_side(&mut connections, &available_level, &level_size, &grid, 0, Side::N);
        scan_width_side(&mut connections, &available_level, &level_size, &grid, level_size.1 - 1, Side::S);

        scan_height_side(&mut connections, &available_level, &level_size, &grid, 0, Side::W);
        scan_height_side(&mut connections, &available_level, &level_size, &grid, level_size.0 - 1, Side::E);

        available_level.connections = connections;

        available_level
    }
}

/*
impl AvailableLevelsPerType {

    fn from_available_level(available_levels: Vec<AvailableLevel>) -> Self {

        let mut per_type = Self { 
            spawning: AvailableLevels::new(),
            regular: AvailableLevels::new(),
        };

        available_levels.into_iter().for_each(|v| {
            if v.level_type == LevelType::Normal {
                per_type.regular.insert(v.level_id.clone(), v);
            } else if v.level_type == LevelType::Spawn {
                per_type.spawning.insert(v.level_id.clone(), v);
            }
        });


        per_type
    }
}
*/

fn populate_level_connections(available_levels: &mut Vec<AvailableLevel>) {

    let mut to_add_elements: Vec<(usize, usize, (AvailableLevel, usize))> = vec![];

    let mut i = 0;
    while i < available_levels.len() {
        let mut y = 0;

        while y < available_levels[i].connections.len() {

            let connection = available_levels[i].connections.get(y).unwrap();

            let mut ii = 1;

            while ii + i < available_levels.len() {

                let mut yy = 0;

                if available_levels[ii + i].level_type == LevelType::Spawn {
                    continue;
                }

                while yy < available_levels[ii + i].connections.len() {


                    let other_connection = available_levels[ii + i].connections.get(yy).unwrap();

                    if connection.are_matching(&other_connection) {

                        to_add_elements.push((i, y, (available_levels[ii + i].clone(), yy)));

                        to_add_elements.push((i + ii, yy, (available_levels[i].clone(), y)));
                    }

                    yy += 1;
                }

                ii += 1;
            }
            
            y += 1;
        }

        i += 1;
    }

    for to_add in to_add_elements {
        available_levels[to_add.0].connections[to_add.1].compatiable_levels.push(to_add.2);
    }

}


pub struct MapGenerationContext {
    pub tile_size: (i32, i32),
    pub level_size: (i32, i32),

    pub available_levels: AvailableLevels,

    pub config: MapGenerationConfig,
}

impl MapGenerationContext {


    pub fn from_map(map_json: &LdtkJson, config: MapGenerationConfig) -> Self {
        if map_json.levels.len() < 1 {
            eprintln!("to few level present in the project");
        }

        let tile_size = (map_json.default_entity_width, map_json.default_entity_height);

        let first_level = map_json.levels.get(0).unwrap();

        let level_size = (
            first_level.px_wid / tile_size.0,
            first_level.px_hei / tile_size.1
        );

        println!("starting level generation for base_map={} \nseed={} \ntilse_size={}x{} \nlevel_size={}x{}\nmap_size={}x{}", 
            config.map_path, config.seed, tile_size.0, tile_size.1, level_size.0, level_size.1,
            config.max_width, config.max_heigth
        );

        let mut available_levels: Vec<AvailableLevel> = map_json.levels.iter()
            .map(|item| {
                AvailableLevel::from_level(&item, &tile_size)
            }).collect();


        populate_level_connections(&mut available_levels);


        Self {
            level_size,
            tile_size,
            config,
            available_levels,
        }
    }


}

use rand::SeedableRng;

pub struct MapGenerationData {
    pub rng: rand::rngs::StdRng,
}

impl MapGenerationData {

    pub fn from_context(context: &MapGenerationContext) -> Self {
        Self {
            rng: rand::rngs::StdRng::seed_from_u64(context.config.seed as u64),
        }
    }
    
}


