use std::fs;

#[macro_use]
extern crate serde;
extern crate serde_json;

mod layout;
use layout::MAPPINGS;

mod keycode_mapping;
use keycode_mapping::process_key_symbol;
use keycode_mapping::TransformedKey;

#[derive(Serialize, Debug)]
struct File {
    rules: Vec<Rule>,
}

#[derive(Serialize, Debug)]
struct Rule {
    description: String,
    manipulators: Vec<Manipulator>,
}

#[derive(Serialize, Debug)]
struct Manipulator {
    from: From,
    to: To,
    #[serde(default)]
    r#type: String,
}

#[derive(Serialize, Debug)]
struct From {
    key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<Modifiers>,
}

#[derive(Serialize, Debug)]
struct To {
    key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    modifiers: Option<Vec<String>>,
}

#[derive(Serialize, Debug)]
struct Modifiers {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    mandatory: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    optional: Vec<String>,
}

fn main() {
    let description = "JIS配列から自作配列への変換".to_string();
    let save_path = "./layout.json";

    let manipulators: Vec<Manipulator> = MAPPINGS
        .iter()
        .map(|(from_input_str, to_input_str)| {
            let from_transformed: TransformedKey = process_key_symbol(from_input_str);
            let from_final_modifiers_obj = if from_transformed.mandatory_modifiers.is_empty() {
                None
            } else {
                Some(Modifiers {
                    mandatory: from_transformed.mandatory_modifiers,
                    optional: Vec::new(),
                })
            };
            let to_transformed: TransformedKey = process_key_symbol(to_input_str);
            let to_final_modifiers_vec = if to_transformed.mandatory_modifiers.is_empty() {
                None
            } else {
                Some(to_transformed.mandatory_modifiers)
            };
            Manipulator {
                from: From {
                    key_code: from_transformed.key_code,
                    modifiers: from_final_modifiers_obj,
                },
                to: To {
                    key_code: to_transformed.key_code,
                    modifiers: to_final_modifiers_vec,
                },
                r#type: "basic".to_string(),
            }
        })
        .collect();
    let config = File {
        rules: vec![Rule {
            description,
            manipulators,
        }],
    };
    let json_str = serde_json::to_string_pretty(&config).unwrap();

    match fs::write(save_path, json_str) {
        Ok(_) => println!("Successfully wrote to {}", save_path),
        Err(e) => eprintln!("Failed to write to {}: {}", save_path, e),
    }
}
