use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct File {
    pub rules: Vec<Rule>,
}

#[derive(Serialize, Debug)]
pub struct Rule {
    pub description: String,
    pub manipulators: Vec<Manipulator>,
}

#[derive(Serialize, Debug)]
pub struct Manipulator {
    pub from: From,
    pub to: Vec<ToEvent>,
    #[serde(default)]
    pub r#type: String,
}

#[derive(Serialize, Debug)]
pub struct From {
    pub key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Modifiers>,
}

#[derive(Serialize, Deserialize,Debug,Clone)]
pub struct ToEvent {
    pub key_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifiers: Option<Vec<String>>,
}

#[derive(Serialize, Debug)]
pub struct Modifiers {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub mandatory: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub optional: Vec<String>,
}
