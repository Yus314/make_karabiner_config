use crate::json_structures::{
    ConditionVariant, File, From, Manipulator, Modifiers, Rule, SimultaneousKey, ToEvent,
};
use crate::keycode_mapping::{
    parse_from_input_string, transform_string_for_to_event, FromEventType, ParsedFromEvent,
    TransformedToKey,
};

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
    global_manipulator_conditions: Option<Vec<ConditionVariant>>,
) -> File {
    let mut final_manipulators: Vec<Manipulator> = Vec::new();

    for (from_input_str_ref, to_input_str_ref) in mappings_to_process.iter() {
        let from_input_str: &str = from_input_str_ref;
        let parsed_from_event: ParsedFromEvent = parse_from_input_string(from_input_str);

        let mut from_object_for_manipulator = crate::json_structures::From::default();

        match parsed_from_event.event_type {
            FromEventType::SingleKey => {
                from_object_for_manipulator.key_code = parsed_from_event.key_code.clone();
                let mandatory_mods = parsed_from_event.modifiers.clone();
                let optional_mods = if set_from_optional_any {
                    vec!["any".to_string()]
                } else {
                    Vec::new()
                };

                if !mandatory_mods.is_empty() || !optional_mods.is_empty() {
                    from_object_for_manipulator.modifiers = Some(Modifiers {
                        mandatory: mandatory_mods,
                        optional: optional_mods,
                    });
                }
            }
            FromEventType::Simultaneous => {
                if let Some(keys) = parsed_from_event.simultaneous_keys {
                    from_object_for_manipulator.simultaneous = Some(
                        keys.into_iter()
                            .map(|kc| SimultaneousKey { key_code: kc })
                            .collect(),
                    );
                }
            }
        }

        let to_input_str: &str = to_input_str_ref;
        let to_transformed_key: TransformedToKey = transform_string_for_to_event(to_input_str);
        let mut to_events_for_manipulator: Vec<ToEvent> = Vec::new();

        let key_code_str_from_transform = to_transformed_key.key_code.clone();
        let modifires_from_transform = if to_transformed_key.mandatory_modifiers.is_empty() {
            None
        } else {
            Some(to_transformed_key.mandatory_modifiers.clone())
        };

        let is_romaji_sequence_to = key_code_str_from_transform.len() > 1
            && key_code_str_from_transform
                .chars()
                .all(|c| c.is_ascii_lowercase())
            && !is_known_single_multichar_keycode(&key_code_str_from_transform);

        if is_romaji_sequence_to {
            for char_in_sequence in key_code_str_from_transform.chars() {
                to_events_for_manipulator.push(ToEvent {
                    key_code: Some(char_in_sequence.to_string()),
                    modifiers: modifires_from_transform.clone(),
                    ..Default::default()
                });
            }
        } else {
            to_events_for_manipulator.push(ToEvent {
                key_code: Some(key_code_str_from_transform.clone()),
                modifiers: modifires_from_transform.clone(),
                ..Default::default()
            });
        }

        final_manipulators.push(Manipulator {
            from: from_object_for_manipulator.clone(),
            to: to_events_for_manipulator.clone(),
            r#type: "basic".to_string(),
            conditions: global_manipulator_conditions.clone(),
        });

	        let should_create_shifted_variant = match parsed_from_event.event_type {
            FromEventType::SingleKey => {
                from_input_str.len() == 1 && from_input_str.chars().all(|c| c.is_ascii_lowercase())
            }
            FromEventType::Simultaneous => {
                true
            }
        };

        if should_create_shifted_variant {
	    let mut shifted_from_object = from_object_for_manipulator;
	                let mut mods = shifted_from_object.modifiers.take().unwrap_or_default();
            mods.mandatory = add_left_shift(&mods.mandatory);
            shifted_from_object.modifiers = Some(mods);

	                let to_transformed_key = transform_string_for_to_event(to_input_str); // 再度取得
            let is_romaji_sequence_to = to_transformed_key.key_code.len() > 1 &&
                                        to_transformed_key.key_code.chars().all(|c| c.is_ascii_lowercase()) &&
                                        !is_known_single_multichar_keycode(&to_transformed_key.key_code);
            let mut to_shifted_events: Vec<ToEvent> = Vec::new();
            let to_shifted_overall_modifiers = add_left_shift(&to_transformed_key.mandatory_modifiers);
	    
            if is_romaji_sequence_to {
                for char_in_sequence in key_code_str_from_transform.chars() {
                    to_shifted_events.push(ToEvent {
                        key_code: Some(char_in_sequence.to_string()),
                        modifiers: Some(to_shifted_overall_modifiers.clone()),
                        ..Default::default()
                    });
                }
            } else {
                to_shifted_events.push(ToEvent {
                    key_code: Some(key_code_str_from_transform.clone()),
                    modifiers: Some(to_shifted_overall_modifiers.clone()),
                    ..Default::default()
                });
            }

            final_manipulators.push(Manipulator {
                from: shifted_from_object,
                to: to_shifted_events,
                r#type: "basic".to_string(),
                conditions: global_manipulator_conditions.clone(),
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
