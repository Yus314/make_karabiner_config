use crate::json_structures::{File, From, Manipulator, Modifiers, Rule, To};
use crate::keycode_mapping::{process_key_symbol, TransformedKey};

fn add_left_shift(current_modifiers: &[String]) -> Vec<String> {
    let mut new_modifiers = current_modifiers.to_vec();
    let shift_key = "left_shift".to_string();
    if !new_modifiers.contains(&shift_key) {
        new_modifiers.push(shift_key);
    }
    new_modifiers
}

pub fn generate_karabiner_config(
    description: String,
    mappings_to_process: &[(String, String)],
    set_from_optional_any: bool,
) -> File {
    let mut final_manipulators: Vec<Manipulator> = Vec::new();

    for (from_input_str_ref, to_input_str_ref) in mappings_to_process.iter() {
        let from_input_str: &str = from_input_str_ref;
        let to_input_str: &str = to_input_str_ref;

        let from_transformed_base: TransformedKey = process_key_symbol(from_input_str);
        let to_transformed_base: TransformedKey = process_key_symbol(to_input_str);

        let from_optional_mods = if set_from_optional_any {
            vec!["any".to_string()]
        } else {
            Vec::new()
        };

        let from_base_modifiers_obj = if from_transformed_base.mandatory_modifiers.is_empty() {
            None
        } else {
            Some(Modifiers {
                mandatory: from_transformed_base.mandatory_modifiers.clone(),
                optional: from_optional_mods.clone(),
            })
        };
        let to_base_modifiers_vec = if to_transformed_base.mandatory_modifiers.is_empty() {
            None
        } else {
            Some(to_transformed_base.mandatory_modifiers.clone())
        };

        final_manipulators.push(Manipulator {
            from: From {
                key_code: from_transformed_base.key_code.clone(),
                modifiers: from_base_modifiers_obj,
            },
            to: To {
                key_code: to_transformed_base.key_code.clone(),
                modifiers: to_base_modifiers_vec,
            },
            r#type: "basic".to_string(),
        });
        if from_input_str.len() == 1 && from_input_str.chars().all(|c| c.is_ascii_lowercase()) {
            let from_shifted_mandatory_mods =
                add_left_shift(&from_transformed_base.mandatory_modifiers);
            let from_shifted_modifiers_obj =
                if from_shifted_mandatory_mods.is_empty() && !set_from_optional_any {
                    None
                } else {
                    Some(Modifiers {
                        mandatory: from_shifted_mandatory_mods,
                        optional: from_optional_mods.clone(),
                    })
                };

            let to_shifted_mandatory_mods =
                add_left_shift(&to_transformed_base.mandatory_modifiers);
            let to_shifted_modifiers_vec = Some(to_shifted_mandatory_mods);

            final_manipulators.push(Manipulator {
                from: From {
                    key_code: from_transformed_base.key_code.clone(),
                    modifiers: from_shifted_modifiers_obj,
                },
                to: To {
                    key_code: to_transformed_base.key_code.clone(),
                    modifiers: to_shifted_modifiers_vec,
                },
                r#type: "basic".to_string(),
            });
        }
    }
    File {
        rules: vec![Rule {
            description,
            manipulators: final_manipulators,
        }],
    }
}
