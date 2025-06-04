use crate::json_structures::{
    ConditionVariant, File, From, InputSourceDetail, Manipulator, Modifiers, Rule, ToEvent,
};
use crate::keycode_mapping::{process_key_symbol, TransformedKey};

fn is_known_single_multichar_keycode(s: &str) -> bool {
    // このリストは、Karabiner Elementsが単一のキーイベントとして認識する
    // 複数文字のキーコード名を含みます。
    // process_key_symbol が返す可能性のあるものを中心に。
    match s {
        "left_control"
        | "right_control"
        | "left_shift"
        | "right_shift"
        | "left_option"
        | "right_option"
        | "left_command"
        | "right_command"
        | "delete_or_backspace"
        | "delete_forward"
        | "escape"
        | "return_or_enter"
        | "spacebar"
        | "tab"
        | "page_up"
        | "page_down"
        | "home"
        | "end"
        | "up_arrow"
        | "down_arrow"
        | "left_arrow"
        | "right_arrow"
        | "f1"
        | "f2"
        | "f3"
        | "f4"
        | "f5"
        | "f6"
        | "f7"
        | "f8"
        | "f9"
        | "f10"
        | "f11"
        | "f12"
        | "f13"
        | "f14"
        | "f15"
        | "f16"
        | "f17"
        | "f18"
        | "f19"
        | "f20"
        | "semicolon"
        | "hyphen"
        | "equal_sign"
        | "open_bracket"
        | "close_bracket"
        | "backslash"
        | "quote"
        | "comma"
        | "period"
        | "slash"
        | "international1"
        | "japanese_eisuu"
        | "japanese_kana"
        | "caps_lock" => true,
        _ => s.starts_with("keypad_") || s.starts_with("vk_"), // 他の一般的なプレフィックス
    }
}

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
    condition_input_source_if_detail: Option<InputSourceDetail>,
) -> File {
    let mut final_manipulators: Vec<Manipulator> = Vec::new();

    for (from_input_str_ref, to_input_str_ref) in mappings_to_process.iter() {
        let from_input_str: &str = from_input_str_ref;
        let from_transformed_base: TransformedKey = process_key_symbol(from_input_str);

        let from_optional_mods_for_from = if set_from_optional_any {
            vec!["any".to_string()]
        } else {
            Vec::new()
        };

        let from_base_modifiers_obj =
            if from_transformed_base.mandatory_modifiers.is_empty() && !set_from_optional_any {
                None
            } else {
                Some(Modifiers {
                    mandatory: from_transformed_base.mandatory_modifiers.clone(),
                    optional: from_optional_mods_for_from.clone(),
                })
            };

        let to_input_str: &str = to_input_str_ref;
        let to_transformed_result: TransformedKey = process_key_symbol(to_input_str);
        let mut to_events_for_manipulator: Vec<ToEvent> = Vec::new();

        let key_code_str_from_transform = to_transformed_result.key_code;
        let modifires_from_transform = if to_transformed_result.mandatory_modifiers.is_empty() {
            None
        } else {
            Some(to_transformed_result.mandatory_modifiers.clone())
        };

        let is_romaji_sequence = key_code_str_from_transform.len() > 1
            && key_code_str_from_transform
                .chars()
                .all(|c| c.is_ascii_lowercase())
            && !is_known_single_multichar_keycode(&key_code_str_from_transform);

        if is_romaji_sequence {
            for char_in_sequence in key_code_str_from_transform.chars() {
                to_events_for_manipulator.push(ToEvent {
                    key_code: char_in_sequence.to_string(),
                    modifiers: modifires_from_transform.clone(),
                });
            }
        } else {
            to_events_for_manipulator.push(ToEvent {
                key_code: key_code_str_from_transform.clone(),
                modifiers: modifires_from_transform.clone(),
            });
        }

        final_manipulators.push(Manipulator {
            from: From {
                key_code: from_transformed_base.key_code.clone(),
                modifiers: from_base_modifiers_obj,
            },
            to: to_events_for_manipulator.clone(),
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
                        optional: from_optional_mods_for_from.clone(),
                    })
                };

            let mut to_shifted_events: Vec<ToEvent> = Vec::new();
            let to_shifted_overall_modifiers =
                add_left_shift(&to_transformed_result.mandatory_modifiers);

            if is_romaji_sequence {
                for char_in_sequence in key_code_str_from_transform.chars() {
                    to_shifted_events.push(ToEvent {
                        key_code: char_in_sequence.to_string(),
                        modifiers: Some(to_shifted_overall_modifiers.clone()),
                    });
                }
            } else {
                to_shifted_events.push(ToEvent {
                    key_code: key_code_str_from_transform.clone(),
                    modifiers: Some(to_shifted_overall_modifiers.clone()),
                });
            }

            final_manipulators.push(Manipulator {
                from: From {
                    key_code: from_transformed_base.key_code.clone(),
                    modifiers: from_shifted_modifiers_obj,
                },
                to: to_shifted_events,
                r#type: "basic".to_string(),
            });
        }
    }
    let mut generated_conditions: Option<Vec<ConditionVariant>> = None;
    if let Some(detail) = condition_input_source_if_detail {
        if detail.input_source_id.is_some() {
            generated_conditions = Some(vec![ConditionVariant::InputSourceIf {
                input_sources: vec![detail],
            }]);
        }
    }
    File {
        rules: vec![Rule {
            description,
            manipulators: final_manipulators,
            conditions: generated_conditions,
        }],
    }
}
