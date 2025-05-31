use std::fs;

#[macro_use]
extern crate serde;
extern crate serde_json;

mod json_structures;
mod karabiner_config_generator;
mod keycode_mapping;
mod layout;

use json_structures::File;
use karabiner_config_generator::generate_karabiner_config;

fn main() {
    let description = "JIS配列から自作配列への変換".to_string();
    let save_path = "./layout.json";

    let config: File = generate_karabiner_config(description);

    let json_str = serde_json::to_string_pretty(&config).unwrap_or_else(|e| {
        eprintln!("Failed to serialize to JSON: {}", e);
        std::process::exit(1);
    });

    match fs::write(save_path, json_str.as_bytes()) {
        Ok(_) => println!("Successfully wrote to {}", save_path),
        Err(e) => {
            eprintln!("Failed to write to {}: {}", save_path, e);
            std::process::exit(1);
        }
    }
}
