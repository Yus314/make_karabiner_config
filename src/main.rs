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

use json_structures::{ConditionVariant, File as KarabinerFile, InputSourceDetail};
use karabiner_config_generator::generate_karabiner_config;
use rust_mappings_parser::parse_mappings_from_rust_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut input_rust_file_path: Option<String> = None;
    let mut output_json_path = "./layout.json".to_string();
    let mut description = "JIS配列から自作配列への変換".to_string();
    let mut set_from_optional_any = false;
    let mut condition_if_input_source_id: Option<String> = None;

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
            "--if-input-source-id" => {
                if i + 1 < args.len() {
                    condition_if_input_source_id = Some(args[i + 1].clone());
                    i += 1;
                } else {
                    eprintln!("Errer: -if-input-source-id requires a value");
                    process::exit(1);
                }
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
    println!(
        "Set 'from.modifiers.optional: [\"any\"]': {}",
        set_from_optional_any
    );
    if condition_if_input_source_id.is_some() {
        println!("Applyinginput_source_if condition with:");
        if let Some(ref id) = condition_if_input_source_id {
            println!(" ID: {}", id);
        }
    }

    let parsed_mappings = match parse_mappings_from_rust_file(&source_rust_file) {
        Ok(mappings) => mappings,
        Err(e) => {
            eprintln!(
                "Error parsing mappings from Rust file '{}': {}",
                source_rust_file, e
            );
            process::exit(1);
        }
    };

    let mut manipulator_conditions: Option<Vec<ConditionVariant>> = None;

    if condition_if_input_source_id.is_some() {
        manipulator_conditions = Some(vec![ConditionVariant::InputSourceIf {
            input_sources: vec![InputSourceDetail {
                input_source_id: condition_if_input_source_id,
            }],
        }]);
    };

    let config: KarabinerFile = generate_karabiner_config(
        description,
        &parsed_mappings,
        set_from_optional_any,
        manipulator_conditions,
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
