use std::fs;

#[macro_use]
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Debug)]
struct Rule {
    description: String,
    manipulators: Vec<Manipulator>,
}

#[derive(Serialize, Debug)]
struct Manipulator {
    from: From,
    to: To,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Serialize, Debug)]
struct From {
    key_code: String,
    modifiers: Modifiers,
}

#[derive(Serialize, Debug)]
struct To {
    key_code: String,
    modifiers: Modifiers,
}

#[derive(Serialize, Debug)]
struct Modifiers {
    mandatory: String,
    optional: String,
}

fn main() {
    let config = Rule {
        description: "test".to_string(),
        manipulators: vec![Manipulator {
            from: From {
                key_code: "a".to_string(),
                modifiers: Modifiers {
                    mandatory: "right_shift".to_string(),
                    optional: "any".to_string(),
                },
            },
            to: To {
                key_code: "b".to_string(),
                modifiers: Modifiers {
                    mandatory: "right_shift".to_string(),
                    optional: "any".to_string(),
                },
            },
            type_: "basic".to_string(),
        }],
    };
    let json_str = serde_json::to_string_pretty(&config).unwrap();
    fs::write("./test.json", json_str);
}
