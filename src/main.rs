use std::env;
use std::fs;
use std::process;

#[macro_use]
extern crate serde;
extern crate serde_json;

mod json_structures;
mod karabiner_config_generator;
mod keycode_mapping;
mod rust_mappings_parser;

use json_structures::File as KarabinerFile;
use karabiner_config_generator::generate_karabiner_config;
use rust_mappings_parser::parse_mappings_from_rust_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input_rust_file_path: Option<String> = None;
    let mut output_json_path = "./layout.json".to_string();
    let mut description = "JIS配列から自作配列への変換".to_string();
    let mut set_from_optional_any = false;

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--input-rs" => {
                if i + 1 < args.len() {
                    input_rust_file_path = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Error: --input-rs requires a file path");
                    process::exit(1);
                }
            }
            "--output" => {
                if i + 1 < args.len() {
                    output_json_path = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --output requires a file path");
                    process::exit(1);
                }
            }
            "--description" => {
                if i + 1 < args.len() {
                    description = args[i + 1].clone();
                    i += 1;
                } else {
                    eprintln!("Error: --description requires a value");
                    process::exit(1);
                }
            }
            "--from-optional-any" => {
                set_from_optional_any = true;
            }
            _ => {}
        }
        i += 1;
    }
    let source_rust_file = match input_rust_file_path {
        Some(path) => path,
        None => {
            eprintln!("Error: Input Rust file path must be specified with --input-rs <path>");
            process::exit(1);
        }
    };
    println!("Reading mappings from: {}", source_rust_file);
    println!("Outputting to: {}", output_json_path);
    println!("Using description: {}", description);
    println!("Set 'from.modifiers.optional: [\"any\"]': {}", set_from_optional_any);


    let parsed_mappings = match parse_mappings_from_rust_file(&source_rust_file) {
        Ok(mappings) => mappings,
        Err(e) => {
            eprintln!("Error parsing mappings from Rust file '{}': {}", source_rust_file, e);
            process::exit(1);
        }
    };
    
    let config: KarabinerFile = generate_karabiner_config(
        description,
        &parsed_mappings,
        set_from_optional_any, 
    );
    let json_str = match serde_json::to_string_pretty(&config) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to serialize to JSON: {}", e);
            std::process::exit(1);
        }
    };

    match fs::write(&output_json_path, json_str.as_bytes()) {
        Ok(_) => println!("Successfully wrote to {}", output_json_path),
        Err(e) => {
            eprintln!("Failed to write to {}: {}", output_json_path, e);
            std::process::exit(1);
        }
    }
}
