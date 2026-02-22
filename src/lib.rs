use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;

// Embed the JSON file at compile time
static PROVIDERS_JSON: &str = include_str!("../providers.json");

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub description: String,
    pub supports_tools: bool,
    pub context_length: Option<u64>,
    pub input_price: f64,
    pub output_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Provider {
    pub label: String,
    pub base_url: String,
    pub models: Vec<Model>,
    pub docs_url: Option<String>,
}

pub type Providers = HashMap<String, Provider>;

pub fn get_providers_data() -> &'static Providers {
    static PROVIDERS: OnceLock<Providers> = OnceLock::new();
    PROVIDERS.get_or_init(|| {
        serde_json::from_str(PROVIDERS_JSON).expect("Failed to parse providers.json")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_providers() {
        let providers = get_providers_data();
        assert!(!providers.is_empty());
        assert!(providers.contains_key("openai"));
    }

    #[tokio::test]
    async fn test_integration_with_llm_connector() {
        let providers = get_providers_data();
        if let Some(openai) = providers.get("openai") {
            assert!(openai.base_url.contains("api.openai.com"));
            let has_gpt4o = openai.models.iter().any(|m| m.id == "gpt-4o");
            assert!(has_gpt4o, "OpenAI provider should have gpt-4o");
        }
    }
}
