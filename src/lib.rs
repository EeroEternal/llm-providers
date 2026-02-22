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

/// 获取所有 Provider 的 ID 列表（排序后）
pub fn list_providers() -> Vec<String> {
    let mut keys: Vec<String> = get_providers_data().keys().cloned().collect();
    keys.sort();
    keys
}

/// 获取指定 Provider 下的所有模型 ID 列表
pub fn list_models(provider_id: &str) -> Option<Vec<String>> {
    get_providers_data().get(provider_id).map(|p| {
        p.models.iter().map(|m| m.id.clone()).collect()
    })
}

/// 根据 Provider ID 和 Model ID 获取模型详细信息
pub fn get_model(provider_id: &str, model_id: &str) -> Option<Model> {
    get_providers_data()
        .get(provider_id)
        .and_then(|p| p.models.iter().find(|m| m.id == model_id).cloned())
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

    #[test]
    fn test_list_providers() {
        let providers = list_providers();
        assert!(providers.contains(&"openai".to_string()));
        assert!(providers.contains(&"anthropic".to_string()));
    }

    #[test]
    fn test_list_models() {
        let models = list_models("openai").expect("OpenAI provider not found");
        assert!(models.contains(&"gpt-4o".to_string()));
    }

    #[test]
    fn test_get_model() {
        let model = get_model("openai", "gpt-4o").expect("Model not found");
        assert_eq!(model.id, "gpt-4o");
        assert!(model.supports_tools);
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
