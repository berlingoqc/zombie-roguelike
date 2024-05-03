mod imp;

pub mod config;
pub mod context;
pub mod entity;
pub mod position;
pub mod room;

use crate::generation::imp::get_implementation;

use self::{
    context::MapGenerationContext,
    entity::{door::DoorConfig, location::EntityLocation, window::WindowConfig},
    room::{Room, RoomConnection},
};

trait IMapGeneration {
    // generate the first room that will be the game starting point
    fn get_spawning_room(&mut self) -> Room;
    // generate the next room and provide the two connection used to create this room
    fn get_next_room(&mut self) -> Option<(Room, RoomConnection, RoomConnection)>;

    fn get_doors(&mut self) -> Vec<(EntityLocation, DoorConfig)>;
    fn get_windows(&mut self) -> Vec<(EntityLocation, WindowConfig)>;
}

pub trait IMapGenerator {
    fn add_room(
        &mut self,
        room: &Room,
        connection_used: Option<&RoomConnection>,
        connected_to: Option<&RoomConnection>,
    );
    fn add_doors(&mut self, doors: &Vec<(EntityLocation, DoorConfig)>);
    fn add_windows(&mut self, windows: &Vec<(EntityLocation, WindowConfig)>);
}

pub fn map_generation(
    context: MapGenerationContext,
    map_generator: &mut impl IMapGenerator,
) -> Result<(), ()> {
    //let mut generated_map = GeneratedMap::create(map_json.levels);
    let mut generator = get_implementation(context);

    // select the spawning room
    let room = generator.get_spawning_room();

    map_generator.add_room(&room, None, None);

    while let Some((next_room, next_room_connection, other_room_connection)) =
        generator.get_next_room()
    {
        map_generator.add_room(
            &next_room,
            Some(&next_room_connection),
            Some(&other_room_connection),
        );
    }

    let doors = generator.get_doors();
    map_generator.add_doors(&doors);

    let windows = generator.get_windows();
    map_generator.add_windows(&windows);

    Ok(())
}
