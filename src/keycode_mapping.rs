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

fn get_hiragana_to_romaji_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        // 清音
        ("あ", "a"),
        ("い", "i"),
        ("う", "u"),
        ("え", "e"),
        ("お", "o"),
        ("か", "ka"),
        ("き", "ki"),
        ("く", "ku"),
        ("け", "ke"),
        ("こ", "ko"),
        ("さ", "sa"),
        ("し", "shi"),
        ("す", "su"),
        ("せ", "se"),
        ("そ", "so"),
        ("た", "ta"),
        ("ち", "chi"),
        ("つ", "tsu"),
        ("て", "te"),
        ("と", "to"),
        ("な", "na"),
        ("に", "ni"),
        ("ぬ", "nu"),
        ("ね", "ne"),
        ("の", "no"),
        ("は", "ha"),
        ("ひ", "hi"),
        ("ふ", "fu"),
        ("へ", "he"),
        ("ほ", "ho"),
        ("ま", "ma"),
        ("み", "mi"),
        ("む", "mu"),
        ("め", "me"),
        ("も", "mo"),
        ("や", "ya"),
        ("ゆ", "yu"),
        ("よ", "yo"),
        ("ら", "ra"),
        ("り", "ri"),
        ("る", "ru"),
        ("れ", "re"),
        ("ろ", "ro"),
        ("わ", "wa"),
        ("ゐ", "wi"),
        ("ゑ", "we"),
        ("を", "wo"), // ゐ (wi), ゑ (we) は現代では稀
        ("ん", "n"),
        // 濁音
        ("が", "ga"),
        ("ぎ", "gi"),
        ("ぐ", "gu"),
        ("げ", "ge"),
        ("ご", "go"),
        ("ざ", "za"),
        ("じ", "ji"),
        ("ず", "zu"),
        ("ぜ", "ze"),
        ("ぞ", "zo"),
        ("だ", "da"),
        ("ぢ", "di"),
        ("づ", "du"),
        ("で", "de"),
        ("ど", "do"), // ぢ (di/ji), づ (du/zu)
        ("ば", "ba"),
        ("び", "bi"),
        ("ぶ", "bu"),
        ("べ", "be"),
        ("ぼ", "bo"),
        // 半濁音
        ("ぱ", "pa"),
        ("ぴ", "pi"),
        ("ぷ", "pu"),
        ("ぺ", "pe"),
        ("ぽ", "po"),
        // 拗音 (きゃ行など)
        ("きゃ", "kya"),
        ("きゅ", "kyu"),
        ("きょ", "kyo"),
        ("しゃ", "sha"),
        ("しゅ", "shu"),
        ("しょ", "sho"), // し (shi) ベース
        ("ちゃ", "cha"),
        ("ちゅ", "chu"),
        ("ちょ", "cho"), // ち (chi) ベース
        ("にゃ", "nya"),
        ("にゅ", "nyu"),
        ("にょ", "nyo"),
        ("ひゃ", "hya"),
        ("ひゅ", "hyu"),
        ("ひょ", "hyo"),
        ("みゃ", "mya"),
        ("みゅ", "myu"),
        ("みょ", "myo"),
        ("りゃ", "rya"),
        ("りゅ", "ryu"),
        ("りょ", "ryo"),
        // 拗音 (ぎゃ行など - 濁音の拗音)
        ("ぎゃ", "gya"),
        ("ぎゅ", "gyu"),
        ("ぎょ", "gyo"),
        ("じゃ", "ja"),
        ("じゅ", "ju"),
        ("じょ", "jo"), // じ (ji) ベース
        ("ぢゃ", "dya"),
        ("ぢゅ", "dyu"),
        ("ぢょ", "dyo"), // ぢ (di) ベース (じゃ、じゅ、じょ との区別)
        ("びゃ", "bya"),
        ("びゅ", "byu"),
        ("びょ", "byo"),
        // 拗音 (ぴゃ行など - 半濁音の拗音)
        ("ぴゃ", "pya"),
        ("ぴゅ", "pyu"),
        ("ぴょ", "pyo"),
        // 小さい「ぁぃぅぇぉ」 (例: ファ (fa) のような外来語表記用)
        // これらは単独でローマ字になるというより、前の文字と組み合わさることが多い。
        // もし単独で処理する必要がある場合 (例: "ぁ" -> "xa" など) は追加する。
        // 通常は IME が "fa" -> "ふぁ" のように処理する。
        // 今回はキー入力シーケンスの生成が目的なので、単独の小さい仮名は含めない方向で。

        // 長音記号
        ("ー", "-"), // キーボードのハイフンと同じになるが、文脈で判断

                     // その他記号 (句読点など) - これらは通常 process_key_symbol の別の箇所で処理されるか、
                     // もしくはキーコードとして直接定義されるべきものが多い。
                     // 必要であれば、特定の記号のローマ字読みのようなものを追加することも可能。
                     // ("。", "."), ("、", ","), ("「", "["), ("」", "]") などは、
                     // get_jis_to_karabiner_map や特別ルールで処理する方が適切。
    ])
}

#[derive(Debug, Default)]
pub struct TransformedKey {
    pub key_code: String,
    pub mandatory_modifiers: Vec<String>,
}

pub fn process_key_symbol(symbol_str: &str) -> TransformedKey {
    let mut current_processing_str = symbol_str.to_string();
    let mut transformed_key = TransformedKey::default();

    if let Some(romaji) = get_hiragana_to_romaji_map().get(symbol_str) {
        current_processing_str = romaji.to_string();
    }

    match current_processing_str.as_str() {
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
    if let Some(kc_str) = convert_jis_symbol_to_keycode_str(&current_processing_str) {
        transformed_key.key_code = kc_str.to_string();
    } else if current_processing_str.len() == 1
        && current_processing_str
            .chars()
            .next()
            .is_some_and(|c| c.is_alphabetic())
    {
        transformed_key.key_code = current_processing_str.to_lowercase();
    } else {
        transformed_key.key_code = current_processing_str;
    }

    transformed_key
}
