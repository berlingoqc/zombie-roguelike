use std::rc::Rc;

use bevy_ecs_ldtk::ldtk::{FieldValue, LayerInstance, LdtkJson, Level};

use crate::{
    generation::{
        config::MapGenerationConfig,
        context::{
            populate_level_connections, scan_height_side, scan_width_side, AvailableLevel,
            LevelType, MapGenerationContext, Side,
        },
        entity::location::{EntityLocation, EntityLocations},
        position::Position,
    },
    ldtk::map_const,
};

fn get_level_field(level: &Level, name: &str) -> Option<FieldValue> {
    level
        .field_instances
        .iter()
        .find(|instance| name == instance.identifier)
        .map(|instance| instance.value.clone())
}

macro_rules! get_entities {
    ($self: expr, $tile_size: expr, $identifier: expr, $type: ident) => {
        $self
            .iter()
            .filter(|x| x.identifier == $identifier)
            .map(|x| {
                let position = Position(x.grid.x, x.grid.y);
                let size = (x.width / $tile_size.0, x.height / $tile_size.1);
                $type {
                    position,
                    size,
                    level_iid: "".to_string(),
                }
            })
            .collect()
    };
}

fn extract_entity_locations(level: &Level, tile_size: &(i32, i32)) -> EntityLocations {
    let entity_layer = level
        .layer_instances
        .as_ref()
        .unwrap()
        .iter()
        .find(|x| x.identifier == map_const::LAYER_ENTITY);

    if let Some(entity_layer) = entity_layer {
        EntityLocations {
            doors: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_DOOR_LOCATION,
                EntityLocation
            ),
            sodas: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_SODA_LOCATION,
                EntityLocation
            ),
            player_spawns: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_PLAYER_SPAWN_LOCATION,
                EntityLocation
            ),
            zombie_spawns: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_ZOMBIE_SPAWN_LOCATION,
                EntityLocation
            ),
            crates: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_CRATE_LOCATION,
                EntityLocation
            ),
            weapons: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_WEAPON_LOCATION,
                EntityLocation
            ),
            windows: get_entities!(
                entity_layer.entity_instances,
                tile_size,
                map_const::ENTITY_WINDOW_LOCATION,
                EntityLocation
            ),
        }
    } else {
        EntityLocations {
            doors: vec![],
            sodas: vec![],
            player_spawns: vec![],
            zombie_spawns: vec![],
            crates: vec![],
            weapons: vec![],
            windows: vec![],
        }
    }
}

fn to_available_level(level: &Level, tile_size: &(i32, i32)) -> AvailableLevel {
    let level_size: (usize, usize) = (
        (level.px_wid / tile_size.0) as usize,
        (level.px_hei / tile_size.1) as usize,
    );

    // identify each level connection
    let connection_layer: &LayerInstance = level
        .layer_instances
        .as_ref()
        .map(|layer_instance| {
            layer_instance
                .into_iter()
                .find(|item| map_const::LAYER_CONNECTION == item.identifier)
                .ok_or_else(|| "Failed to find LevelConnetion Layer on level")
        })
        .unwrap_or_else(|| Err("No Layers present"))
        .unwrap();

    let grid: Vec<&[i32]> = connection_layer
        .int_grid_csv
        .chunks(level_size.0 as usize)
        .collect();

    let level_type = {
        let is_spawn = get_level_field(&level, map_const::LEVEL_FIELD_SPAWN).map_or(false, |x| {
            if let FieldValue::Bool(value) = x {
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

    // get my entity layer from the level and extract all entity

    let mut available_level = AvailableLevel {
        level_id: level.identifier.clone(),
        connections: vec![],
        level_size,
        level_size_p: (
            level_size.0 as i32 * tile_size.0,
            level_size.1 as i32 * tile_size.1,
        ),
        level_type,
        entity_locations: extract_entity_locations(level, tile_size),
    };

    let mut connections = vec![];
    let mut index = 0;
    scan_width_side(
        &mut connections,
        &mut index,
        &available_level,
        &level_size,
        &grid,
        0,
        Side::N,
    );
    scan_width_side(
        &mut connections,
        &mut index,
        &available_level,
        &level_size,
        &grid,
        level_size.1 - 1,
        Side::S,
    );

    scan_height_side(
        &mut connections,
        &mut index,
        &available_level,
        &level_size,
        &grid,
        0,
        Side::W,
    );
    scan_height_side(
        &mut connections,
        &mut index,
        &available_level,
        &level_size,
        &grid,
        level_size.0 - 1,
        Side::E,
    );

    available_level.connections = connections;

    available_level
}

pub fn from_map(map_json: &LdtkJson, config: MapGenerationConfig) -> MapGenerationContext {
    if map_json.levels.len() < 1 {
        eprintln!("to few level present in the project");
    }

    let tile_size = (
        map_json.default_entity_width,
        map_json.default_entity_height,
    );

    let first_level = map_json.levels.get(0).unwrap();

    let level_size = (
        first_level.px_wid / tile_size.0,
        first_level.px_hei / tile_size.1,
    );

    println!("starting level generation with config \nseed={} \ntilse_size={}x{} \nlevel_size={}x{}\nmap_size={}x{}", 
        config.seed, tile_size.0, tile_size.1, level_size.0, level_size.1,
        config.max_width, config.max_heigth
    );

    let mut available_levels: Vec<AvailableLevel> = map_json
        .levels
        .iter()
        .map(|item| to_available_level(&item, &tile_size))
        .collect();

    populate_level_connections(&mut available_levels);

    let available_levels = available_levels
        .iter()
        .map(|x| Rc::new(x.clone()))
        .collect();

    MapGenerationContext {
        level_size,
        tile_size,
        config,
        available_levels,
    }
}
