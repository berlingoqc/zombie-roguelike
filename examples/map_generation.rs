
use bevy_ecs_ldtk::ldtk::LdtkJson;
use serde_json::{from_str, to_string_pretty};
use std::fs::File;
use std::io::{Read, Write};

use map::generation::{map_generation, config::MapGenerationConfig};


fn main() {

    let usage = "Usage: <base_map_path> <output_map_path> [seed]";

    // Get the file path from the command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("{}", usage);
        std::process::exit(1);
    }

    let map_path = &args[1];
    let map_output_path = &args[2];

    let default_seed = 1; 

    // Parse the second argument as a number or use the default value
    let seed: i32 = args.get(3).map_or(default_seed, |arg| {
        arg.parse().unwrap_or_else(|_| {
            eprintln!("Error parsing the seed argument. Using default value: {}", default_seed);
            default_seed
        })
    });

    println!("loading base map {}", map_path);

        // Open the file
    let mut file = File::open(map_path).expect("Failed to open file");

    // Read the file content into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    // Deserialize the JSON string into your data structure
    let data: LdtkJson = from_str(&contents).expect("Failed to deserialize JSON");


    let data: LdtkJson = map_generation(data.clone(), MapGenerationConfig{seed, ..Default::default()}).expect("Failed to generate map");

        // Convert JSON data to a pretty formatted string
    let pretty_json_string = to_string_pretty(&data).expect("Failed to serialize JSON");

    // Open the file for writing
    let mut file = File::create(map_output_path).expect("Failed to create file");

    // Write the pretty formatted JSON string to the file
    file.write_all(pretty_json_string.as_bytes())
        .expect("Failed to write to file");

    println!("Generated map written to {}", map_output_path);


}

