
use std::ops::RangeInclusive;

use bevy::prelude::Resource;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MapGenerationMode {
    Basic,
}

#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct MapGenerationConfig {

    pub map_path: String,

    pub seed: i32,

    pub max_width: i32,
    pub max_heigth: i32,

    pub mode: MapGenerationMode,
}

impl Default for MapGenerationConfig {
    fn default() -> Self {
        Self { seed: 1, max_width: 255, max_heigth: 255, map_path: "".into(),  mode: MapGenerationMode::Basic }
    }
}


impl MapGenerationConfig {
    pub fn get_range_x(&self, my_size: i32) -> RangeInclusive<i32> {
        -self.max_width..=(self.max_width - my_size)
    }

    pub fn get_range_y(&self, my_size: i32) -> RangeInclusive<i32> {
        -self.max_heigth..=(self.max_heigth - my_size)
    }
    
}
