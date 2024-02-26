
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use utils::camera::tod::{move_camera, setup_camera};
use map::{loader::{setup_generated_map, get_asset_loader_generation}, generation::config::MapGenerationConfig};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use std::env;

fn main() {

    let level_loader = get_asset_loader_generation();

    let map_generation_config = get_config();

    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(LdtkPlugin)
        .add_systems(Startup, setup_generated_map)
        .add_systems(Startup, setup_camera)
        .insert_resource(LevelSelection::index(0))
        .insert_resource(LdtkSettings{
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { load_level_neighbors: true},
            ..default()
        })
        .insert_resource(map_generation_config)
        .register_asset_loader(level_loader)
        .add_systems(
            Update,
            (
                move_camera,
            ),
        )
        .run();

}

fn get_config() -> MapGenerationConfig {
    let args: Vec<String> = env::args().collect();

    let map_path = if args.len() < 2 {
        "exemples/test_map.ldtk".to_string()
    } else {
        args.get(1).unwrap().clone()
    };

    let seed = if args.len() < 3 {
        12345
    } else {
        args.get(2).unwrap().parse::<i32>().unwrap()
    };

    MapGenerationConfig {
        seed,
        map_path,
        max_width: 1000,
        max_heigth: 1000,
        ..Default::default()
    }
}


