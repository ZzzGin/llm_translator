use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TranslatorOutput {
    pub original: String,
    pub fixed: String,
    pub translated: String,
    pub typo_map: HashMap<String, String>,
}
