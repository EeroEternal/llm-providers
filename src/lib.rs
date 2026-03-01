use serde::Serialize;

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Model {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub supports_tools: bool,
    pub context_length: Option<u64>,
    pub input_price: f64,
    pub output_price: f64,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Endpoint {
    pub label: &'static str,
    pub region: &'static str,
    pub base_url: &'static str,
    pub price_currency: &'static str,
    pub docs_url: Option<&'static str>,
}

#[derive(Debug, Serialize)]
pub struct Provider {
    pub label: &'static str,
    pub endpoints: &'static phf::Map<&'static str, Endpoint>,
    pub models: &'static [Model],
}

include!(concat!(env!("OUT_DIR"), "/registry_generated.rs"));

pub type Providers = phf::Map<&'static str, Provider>;

/// Get the providers map
pub fn get_providers_data() -> &'static Providers {
    &PROVIDERS
}

pub fn registry_version() -> &'static str {
    REGISTRY_VERSION
}

pub fn registry_updated_at() -> &'static str {
    REGISTRY_UPDATED_AT
}

/// List all provider family IDs (sorted)
pub fn list_providers() -> Vec<String> {
    let mut keys: Vec<String> = get_providers_data()
        .keys()
        .cloned()
        .map(|s| s.to_string())
        .collect();
    keys.sort();
    keys
}

/// List all endpoint IDs (sorted)
pub fn list_endpoints() -> Vec<String> {
    let mut ids: Vec<String> = get_providers_data()
        .entries()
        .flat_map(|(provider_id, p)| {
            p.endpoints
                .keys()
                .cloned()
                .map(move |endpoint_key| format!("{}:{}", provider_id, endpoint_key))
        })
        .collect();
    ids.sort();
    ids
}

/// Find endpoint details by endpoint ID, returning (family_id, Endpoint)
///
/// Endpoint ID format: "{provider_id}:{endpoint_key}".
pub fn get_endpoint(endpoint_id: &str) -> Option<(&'static str, &'static Endpoint)> {
    if let Some((provider_id, endpoint_key)) = endpoint_id.split_once(':') {
        for (&pid, provider) in get_providers_data() {
            if pid == provider_id {
                let ep = provider.endpoints.get(endpoint_key)?;
                return Some((pid, ep));
            }
        }
        return None;
    }

    let mut found: Option<(&'static str, &'static Endpoint)> = None;
    for (&family_id, provider) in get_providers_data() {
        if let Some(ep) = provider.endpoints.get(endpoint_id) {
            if found.is_some() {
                return None;
            }
            found = Some((family_id, ep));
        }
    }
    found
}

/// List all model IDs under a provider family
pub fn list_models(provider_id: &str) -> Option<Vec<String>> {
    get_providers_data()
        .get(provider_id)
        .map(|p| p.models.iter().map(|m| m.id.to_string()).collect())
}

/// Get model details by (provider_id, model_id)
pub fn get_model(provider_id: &str, model_id: &str) -> Option<Model> {
    get_model_ref(provider_id, model_id).copied()
}

pub fn get_model_ref(provider_id: &str, model_id: &str) -> Option<&'static Model> {
    get_providers_data()
        .get(provider_id)
        .and_then(|p| p.models.iter().find(|m| m.id == model_id))
}

#[derive(Default)]
pub struct ModelFilter {
    pub provider_id: Option<String>,
    pub region: Option<String>,
    pub supports_tools: Option<bool>,
    pub min_context_length: Option<u64>,
}

/// Advanced filtering: returns a list of (provider_id, Model)
pub fn filter_models(filter: ModelFilter) -> Vec<(String, Model)> {
    let mut results = Vec::new();

    for (&pid, provider) in get_providers_data() {
        // Filter by provider_id
        if let Some(ref target_pid) = filter.provider_id {
            if pid != target_pid.as_str() {
                continue;
            }
        }

        // Filter by region: match if any endpoint has the target region
        if let Some(ref target_region) = filter.region {
            let has_region = provider
                .endpoints
                .values()
                .any(|ep| ep.region == target_region);
            if !has_region {
                continue;
            }
        }

        for model in provider.models {
            // Filter by supports_tools
            if let Some(target_tools) = filter.supports_tools {
                if model.supports_tools != target_tools {
                    continue;
                }
            }

            // Filter by min_context_length
            if let Some(min_ctx) = filter.min_context_length {
                if model.context_length.unwrap_or(0) < min_ctx {
                    continue;
                }
            }

            results.push((pid.to_string(), *model));
        }
    }

    // Sort for deterministic output (by provider_id, then model_id)
    results.sort_by(|a, b| {
        let p_cmp = a.0.cmp(&b.0);
        if p_cmp == std::cmp::Ordering::Equal {
            a.1.id.cmp(&b.1.id)
        } else {
            p_cmp
        }
    });

    results
}

pub fn filter_models_ref(filter: ModelFilter) -> Vec<(&'static str, &'static Model)> {
    let mut results = Vec::new();

    for (&pid, provider) in get_providers_data() {
        if let Some(ref target_pid) = filter.provider_id {
            if pid != target_pid.as_str() {
                continue;
            }
        }

        if let Some(ref target_region) = filter.region {
            let has_region = provider
                .endpoints
                .values()
                .any(|ep| ep.region == target_region);
            if !has_region {
                continue;
            }
        }

        for model in provider.models {
            if let Some(target_tools) = filter.supports_tools {
                if model.supports_tools != target_tools {
                    continue;
                }
            }

            if let Some(min_ctx) = filter.min_context_length {
                if model.context_length.unwrap_or(0) < min_ctx {
                    continue;
                }
            }

            results.push((pid, model));
        }
    }

    results.sort_by(|a, b| {
        let p_cmp = a.0.cmp(b.0);
        if p_cmp == std::cmp::Ordering::Equal {
            a.1.id.cmp(b.1.id)
        } else {
            p_cmp
        }
    });

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_providers() {
        let providers = get_providers_data();
        assert!(!providers.is_empty());
        assert!(providers.contains_key("openai"));

        assert_eq!(registry_version(), "2.0");
    }

    #[test]
    fn test_provider_endpoints() {
        let providers = get_providers_data();
        let aliyun = providers.get("aliyun").expect("aliyun not found");
        let ep = aliyun
            .endpoints
            .get("cn")
            .expect("aliyun endpoint not found");
        assert_eq!(ep.region, "cn");
        assert_eq!(ep.price_currency, "CNY");

        // Multi-endpoint provider
        let moonshot = providers.get("moonshot").expect("moonshot not found");
        assert!(moonshot.endpoints.contains_key("cn"));
        assert!(moonshot.endpoints.contains_key("global"));
        assert_eq!(moonshot.endpoints["cn"].region, "cn");
        assert_eq!(moonshot.endpoints["global"].region, "global");
    }

    #[test]
    fn test_list_providers() {
        let providers = list_providers();
        assert!(providers.contains(&"openai".to_string()));
        assert!(providers.contains(&"anthropic".to_string()));
        // moonshot_global should NOT be a top-level provider anymore
        assert!(!providers.contains(&"moonshot_global".to_string()));
    }

    #[test]
    fn test_list_endpoints() {
        let endpoints = list_endpoints();
        assert!(endpoints.contains(&"openai:global".to_string()));
        assert!(endpoints.contains(&"moonshot:cn".to_string()));
        assert!(endpoints.contains(&"moonshot:global".to_string()));
    }

    #[test]
    fn test_get_endpoint() {
        let (family_id, ep) = get_endpoint("moonshot:global").expect("endpoint not found");
        assert_eq!(family_id, "moonshot");
        assert_eq!(ep.region, "global");
        assert_eq!(ep.price_currency, "USD");
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

    #[test]
    fn test_filter_models() {
        // 1. Filter by region="cn" — matches providers that have any CN endpoint
        let cn_models = filter_models(ModelFilter {
            region: Some("cn".to_string()),
            ..Default::default()
        });
        assert!(!cn_models.is_empty());
        assert!(cn_models.iter().any(|(p, _)| p == "aliyun"));
        assert!(!cn_models.iter().any(|(p, _)| p == "openai"));

        // 2. Filter by supports_tools=true
        let tool_models = filter_models(ModelFilter {
            supports_tools: Some(true),
            provider_id: Some("openai".to_string()),
            ..Default::default()
        });
        assert!(tool_models.iter().any(|(_, m)| m.id == "gpt-4o"));
    }

    #[tokio::test]
    async fn test_integration_with_llm_connector() {
        let providers = get_providers_data();
        if let Some(openai) = providers.get("openai") {
            let ep = openai
                .endpoints
                .get("global")
                .expect("openai endpoint not found");
            assert!(ep.base_url.contains("api.openai.com"));
            let has_gpt4o = openai.models.iter().any(|m| m.id == "gpt-4o");
            assert!(has_gpt4o, "OpenAI provider should have gpt-4o");
        }
    }
}
