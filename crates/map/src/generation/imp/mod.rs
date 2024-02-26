
use self::basic::BasicMapGeneration;

use super::{config:: MapGenerationMode, IMapGeneration, context::MapGenerationContext};

mod basic;

pub fn get_implementation(context: MapGenerationContext) -> Box<dyn IMapGeneration> {
    match context.config.mode {
        MapGenerationMode::Basic => Box::new(BasicMapGeneration::create(context))
    }
}
