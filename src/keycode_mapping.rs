use std::collections::HashMap;

fn get_jis_to_karabiner_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("-", "hyphen"),
        (",", "comma"),
        (".", "period"),
        ("/", "slash"),
        ("=", "equal_sign"),
        ("@", "open_bracket"),
        ("[", "close_bracket"),
        ("]", "backslash"),
        (";", "semicolon"),
        (":", "quote"),
        ("_", "international1"),
    ])
}

pub fn convert_jis_symbol_to_keycode_str(jis_symbol: &str) -> Option<&'static str> {
    get_jis_to_karabiner_map().get(jis_symbol).copied()
}

#[derive(Debug, Default)]
pub struct TransformedKey {
    pub key_code: String,
    pub mandatory_modifiers: Vec<String>,
}

pub fn process_key_symbol(symbol_str: &str) -> TransformedKey {
    let mut transformed_key = TransformedKey::default();
    match symbol_str {
        "=" => {
            transformed_key.key_code = convert_jis_symbol_to_keycode_str("-")
                .unwrap_or("-")
                .to_string();
            transformed_key
                .mandatory_modifiers
                .push("left_shift".to_string());
            return transformed_key;
        }
        "'" => {
            transformed_key.key_code = convert_jis_symbol_to_keycode_str("7")
                .unwrap_or("7")
                .to_string();
            transformed_key
                .mandatory_modifiers
                .push("left_shift".to_string());
            return transformed_key;
        }
        _ => {}
    }
    if let Some(kc_str) = convert_jis_symbol_to_keycode_str(symbol_str) {
        transformed_key.key_code = kc_str.to_string();
    } else {
        transformed_key.key_code = symbol_str.to_string();
    }
    transformed_key
}
