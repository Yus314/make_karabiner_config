use std::fs;

#[macro_use]
extern crate serde;
extern crate serde_json;

mod layout;
use layout::MAPPINGS;

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
    modifiers: Option<Modifiers>,
}

#[derive(Serialize, Debug)]
struct Modifiers {
    mandatory: String,
    optional: String,
}

fn main() {
    let description = "JIS配列から自作配列への変換".to_string();
    let save_path = "./layout.json";
    let config = File {
        rules: vec![Rule {
            description: description,
            manipulators: MAPPINGS
                .into_iter()
                .map(|(from_key, to_key)| Manipulator {
                    from: From {
                        key_code: from_key.to_string(),
                        modifiers: None,
                    },
                    to: To {
                        key_code: to_key.to_string(),
                        modifiers: None,
                    },
                    type_: "basic".to_string(),
                })
                .collect(),
        }],
    };
    let json_str = serde_json::to_string_pretty(&config).unwrap();
    fs::write(save_path, json_str);
}
