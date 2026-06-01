mod export;

use serde::Serialize;

pub use export::{
    CatalogEntry, ExportRegistry, GlobalPricing, ModelOffering, export_pararouter_registry,
};

#[derive(Debug, Serialize, Clone, Copy)]
pub struct Model {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub supports_tools: bool,
    pub supports_vision: bool,
    pub supports_reasoning: bool,
    pub context_length: Option<u64>,
    pub input_price: f64,
    pub output_price: f64,
    pub cache_read_price: Option<f64>,
    pub cache_write_price: Option<f64>,
    pub reasoning_price: Option<f64>,
    pub category: Option<&'static str>,
    pub published_at: Option<&'static str>,
    pub deprecated_at: Option<&'static str>,
    pub replacement_id: Option<&'static str>,
}

impl Model {
    pub fn is_deprecated(&self) -> bool {
        self.deprecated_at.is_some()
    }
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
    pub endpoint_models: Option<&'static phf::Map<&'static str, &'static [Model]>>,
}

/// Model resolved at a specific endpoint, including regional pricing currency.
#[derive(Debug, Serialize, Clone, Copy)]
pub struct ResolvedModel {
    pub provider_id: &'static str,
    pub endpoint_key: &'static str,
    pub model: Model,
    pub price_currency: &'static str,
    pub region: &'static str,
    pub base_url: &'static str,
}

impl ResolvedModel {
    pub fn endpoint_id(&self) -> String {
        format!("{}:{}", self.provider_id, self.endpoint_key)
    }
}

include!(concat!(env!("OUT_DIR"), "/registry_generated.rs"));

pub type Providers = phf::Map<&'static str, Provider>;

const RESELLER_PROVIDER_IDS: &[&str] =
    &["tencent", "volcengine", "openrouter", "baseten", "zenmux"];

pub fn get_providers_data() -> &'static Providers {
    &PROVIDERS
}

pub fn registry_version() -> &'static str {
    REGISTRY_VERSION
}

pub fn registry_updated_at() -> &'static str {
    REGISTRY_UPDATED_AT
}

/// Strip aggregator-style prefixes (`google/gemini-3.5-flash` → `gemini-3.5-flash`).
pub fn canonical_model_id(model_id: &str) -> &str {
    model_id.rsplit('/').next().unwrap_or(model_id)
}

pub fn is_reseller_provider(provider_id: &str) -> bool {
    RESELLER_PROVIDER_IDS.contains(&provider_id)
}

pub(crate) fn provider_catalog_priority(provider_id: &str) -> u8 {
    if is_reseller_provider(provider_id) {
        100
    } else {
        0
    }
}

pub fn list_providers() -> Vec<String> {
    let mut keys: Vec<String> = get_providers_data()
        .keys()
        .cloned()
        .map(|s| s.to_string())
        .collect();
    keys.sort();
    keys
}

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

pub fn list_models(provider_id: &str) -> Option<Vec<String>> {
    get_providers_data()
        .get(provider_id)
        .map(|p| p.models.iter().map(|m| m.id.to_string()).collect())
}

pub fn list_models_for_endpoint(endpoint_id: &str) -> Option<Vec<String>> {
    list_offerings_for_endpoint(endpoint_id).map(|offerings| {
        offerings
            .into_iter()
            .map(|o| o.model.id.to_string())
            .collect()
    })
}

pub fn get_model(provider_id: &str, model_id: &str) -> Option<Model> {
    get_model_ref(provider_id, model_id).copied()
}

pub fn get_model_ref(provider_id: &str, model_id: &str) -> Option<&'static Model> {
    get_providers_data()
        .get(provider_id)
        .and_then(|p| p.models.iter().find(|m| m.id == model_id))
}

pub fn get_model_for_endpoint(endpoint_id: &str, model_id: &str) -> Option<ResolvedModel> {
    resolve_offering(endpoint_id, model_id)
}

fn models_for_endpoint_provider(provider: &Provider, endpoint_key: &str) -> &'static [Model] {
    if let Some(endpoint_models) = provider.endpoint_models
        && let Some(models) = endpoint_models.get(endpoint_key)
    {
        return models;
    }
    provider.models
}

fn resolve_offering(endpoint_id: &str, model_id: &str) -> Option<ResolvedModel> {
    let (provider_id, endpoint_key_part) = endpoint_id.split_once(':')?;
    let (&pid, provider) = get_providers_data()
        .entries()
        .find(|(id, _)| **id == provider_id)?;
    let (&endpoint_key, ep) = provider
        .endpoints
        .entries()
        .find(|(key, _)| **key == endpoint_key_part)?;
    let model = models_for_endpoint_provider(provider, endpoint_key)
        .iter()
        .find(|m| m.id == model_id)
        .copied()?;

    Some(ResolvedModel {
        provider_id: pid,
        endpoint_key,
        model,
        price_currency: ep.price_currency,
        region: ep.region,
        base_url: ep.base_url,
    })
}

/// Every deployable `(provider, endpoint, model)` tuple with endpoint-accurate pricing.
pub fn list_offerings() -> Vec<ModelOffering> {
    let mut offerings = Vec::new();

    for (&provider_id, provider) in get_providers_data() {
        for (&endpoint_key, ep) in provider.endpoints.entries() {
            for model in models_for_endpoint_provider(provider, endpoint_key) {
                offerings.push(ModelOffering {
                    provider_id,
                    endpoint_key,
                    endpoint_id: format!("{provider_id}:{endpoint_key}"),
                    model: *model,
                    price_currency: ep.price_currency,
                    region: ep.region,
                    base_url: ep.base_url,
                });
            }
        }
    }

    offerings.sort_by(|a, b| {
        a.endpoint_id
            .cmp(&b.endpoint_id)
            .then_with(|| a.model.id.cmp(b.model.id))
    });
    offerings
}

fn list_offerings_for_endpoint(endpoint_id: &str) -> Option<Vec<ModelOffering>> {
    let (provider_id, endpoint_key_part) = endpoint_id.split_once(':')?;
    let (&pid, provider) = get_providers_data()
        .entries()
        .find(|(id, _)| **id == provider_id)?;
    let (&endpoint_key, ep) = provider
        .endpoints
        .entries()
        .find(|(key, _)| **key == endpoint_key_part)?;

    let mut offerings = models_for_endpoint_provider(provider, endpoint_key)
        .iter()
        .map(|model| ModelOffering {
            provider_id: pid,
            endpoint_key,
            endpoint_id: endpoint_id.to_string(),
            model: *model,
            price_currency: ep.price_currency,
            region: ep.region,
            base_url: ep.base_url,
        })
        .collect::<Vec<_>>();

    offerings.sort_by(|a, b| a.model.id.cmp(b.model.id));
    Some(offerings)
}

/// Global catalog keyed by canonical model id. Reseller/duplicate ids collapse to one entry.
pub fn list_catalog_models() -> Vec<CatalogEntry> {
    export::build_catalog_from_offerings(&list_offerings())
}

/// Back-compat alias for [`list_catalog_models`].
pub fn list_models_unique() -> Vec<CatalogEntry> {
    list_catalog_models()
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum OrderBy {
    #[default]
    ProviderAndModelId,
    PublishedAtAsc,
    PublishedAtDesc,
}

#[derive(Clone)]
pub struct ModelFilter {
    pub provider_id: Option<String>,
    pub endpoint_id: Option<String>,
    pub region: Option<String>,
    pub supports_tools: Option<bool>,
    pub supports_vision: Option<bool>,
    pub supports_reasoning: Option<bool>,
    pub min_context_length: Option<u64>,
    pub exclude_deprecated: bool,
    pub order_by: Option<OrderBy>,
}

impl Default for ModelFilter {
    fn default() -> Self {
        Self {
            provider_id: None,
            endpoint_id: None,
            region: None,
            supports_tools: None,
            supports_vision: None,
            supports_reasoning: None,
            min_context_length: None,
            exclude_deprecated: true,
            order_by: None,
        }
    }
}

fn offering_matches_filter(offering: &ModelOffering, filter: &ModelFilter) -> bool {
    if let Some(ref target_pid) = filter.provider_id
        && offering.provider_id != target_pid.as_str()
    {
        return false;
    }

    if let Some(ref target_eid) = filter.endpoint_id
        && offering.endpoint_id != *target_eid
    {
        return false;
    }

    if let Some(ref target_region) = filter.region
        && offering.region != target_region.as_str()
    {
        return false;
    }

    if let Some(target_tools) = filter.supports_tools
        && offering.model.supports_tools != target_tools
    {
        return false;
    }

    if let Some(target_vision) = filter.supports_vision
        && offering.model.supports_vision != target_vision
    {
        return false;
    }

    if let Some(target_reasoning) = filter.supports_reasoning
        && offering.model.supports_reasoning != target_reasoning
    {
        return false;
    }

    if let Some(min_ctx) = filter.min_context_length
        && offering.model.context_length.unwrap_or(0) < min_ctx
    {
        return false;
    }

    if filter.exclude_deprecated && offering.model.is_deprecated() {
        return false;
    }

    true
}

fn sort_offerings(results: &mut [ModelOffering], order_by: Option<OrderBy>) {
    match order_by {
        Some(OrderBy::PublishedAtAsc) => {
            results.sort_by(|a, b| match (a.model.published_at, b.model.published_at) {
                (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a
                    .endpoint_id
                    .cmp(&b.endpoint_id)
                    .then_with(|| a.model.id.cmp(b.model.id)),
            });
        }
        Some(OrderBy::PublishedAtDesc) => {
            results.sort_by(|a, b| match (a.model.published_at, b.model.published_at) {
                (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a
                    .endpoint_id
                    .cmp(&b.endpoint_id)
                    .then_with(|| a.model.id.cmp(b.model.id)),
            });
        }
        _ => {
            results.sort_by(|a, b| {
                a.endpoint_id
                    .cmp(&b.endpoint_id)
                    .then_with(|| a.model.id.cmp(b.model.id))
            });
        }
    }
}

/// Endpoint-aware filtering; each result includes regional pricing currency.
pub fn filter_offerings(filter: ModelFilter) -> Vec<ModelOffering> {
    let mut results: Vec<ModelOffering> = list_offerings()
        .into_iter()
        .filter(|offering| offering_matches_filter(offering, &filter))
        .collect();

    sort_offerings(&mut results, filter.order_by);
    results
}

/// Deprecated: use [`filter_offerings`] for endpoint-accurate pricing and currency.
pub fn filter_models(filter: ModelFilter) -> Vec<(String, Model)> {
    filter_offerings(filter)
        .into_iter()
        .map(|o| (o.provider_id.to_string(), o.model))
        .collect()
}

pub fn filter_models_ref(filter: ModelFilter) -> Vec<(&'static str, &'static Model)> {
    let mut results = Vec::new();

    for (&pid, provider) in get_providers_data() {
        if let Some(ref target_pid) = filter.provider_id
            && pid != target_pid.as_str()
        {
            continue;
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
            if filter.exclude_deprecated && model.is_deprecated() {
                continue;
            }
            if let Some(target_tools) = filter.supports_tools
                && model.supports_tools != target_tools
            {
                continue;
            }
            if let Some(target_vision) = filter.supports_vision
                && model.supports_vision != target_vision
            {
                continue;
            }
            if let Some(target_reasoning) = filter.supports_reasoning
                && model.supports_reasoning != target_reasoning
            {
                continue;
            }
            if let Some(min_ctx) = filter.min_context_length
                && model.context_length.unwrap_or(0) < min_ctx
            {
                continue;
            }

            results.push((pid, model));
        }
    }

    match filter.order_by {
        Some(OrderBy::PublishedAtAsc) => {
            results.sort_by(|a, b| match (a.1.published_at, b.1.published_at) {
                (Some(a_date), Some(b_date)) => a_date.cmp(b_date),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => {
                    let p_cmp = a.0.cmp(b.0);
                    if p_cmp == std::cmp::Ordering::Equal {
                        a.1.id.cmp(b.1.id)
                    } else {
                        p_cmp
                    }
                }
            });
        }
        Some(OrderBy::PublishedAtDesc) => {
            results.sort_by(|a, b| match (a.1.published_at, b.1.published_at) {
                (Some(a_date), Some(b_date)) => b_date.cmp(a_date),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => {
                    let p_cmp = a.0.cmp(b.0);
                    if p_cmp == std::cmp::Ordering::Equal {
                        a.1.id.cmp(b.1.id)
                    } else {
                        p_cmp
                    }
                }
            });
        }
        _ => {
            results.sort_by(|a, b| {
                let p_cmp = a.0.cmp(b.0);
                if p_cmp == std::cmp::Ordering::Equal {
                    a.1.id.cmp(b.1.id)
                } else {
                    p_cmp
                }
            });
        }
    }

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
        assert_eq!(registry_version(), "3.0");
    }

    #[test]
    fn test_canonical_model_id() {
        assert_eq!(canonical_model_id("gemini-3.5-flash"), "gemini-3.5-flash");
        assert_eq!(
            canonical_model_id("google/gemini-3.5-flash"),
            "gemini-3.5-flash"
        );
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
    }

    #[test]
    fn test_get_model_for_endpoint_includes_currency() {
        let resolved =
            get_model_for_endpoint("minimax:global", "MiniMax-M3").expect("model not found");
        assert_eq!(resolved.price_currency, "USD");
        assert_eq!(resolved.model.input_price, 0.6);
    }

    #[test]
    fn test_list_catalog_dedupes_resellers() {
        let catalog = list_catalog_models();
        let flash = catalog
            .iter()
            .find(|c| c.id == "deepseek-v4-flash")
            .expect("catalog entry");
        assert_eq!(flash.primary_provider_id, "deepseek");
    }

    #[test]
    fn test_list_offerings_includes_tencent() {
        let offerings = list_offerings();
        assert!(
            offerings
                .iter()
                .any(|o| o.provider_id == "tencent" && o.model.id == "deepseek-v4-flash")
        );
    }

    #[test]
    fn test_filter_offerings_excludes_deprecated_by_default() {
        let all = filter_offerings(ModelFilter {
            exclude_deprecated: false,
            provider_id: Some("deepseek".to_string()),
            ..Default::default()
        });
        let active = filter_offerings(ModelFilter {
            provider_id: Some("deepseek".to_string()),
            ..Default::default()
        });
        assert!(active.len() < all.len() || all.iter().all(|o| !o.model.is_deprecated()));
    }

    #[test]
    fn test_export_pararouter_shape() {
        let exported = export_pararouter_registry();
        assert_eq!(exported.registry_version, registry_version());
        assert!(!exported.catalog.is_empty());
        assert!(!exported.offerings.is_empty());
        assert!(
            exported
                .offerings
                .iter()
                .all(|o| !o.price_currency.is_empty())
        );
    }

    #[test]
    fn test_get_model() {
        let model = get_model("openai", "gpt-4o").expect("Model not found");
        assert_eq!(model.id, "gpt-4o");
        assert!(model.supports_tools);
    }
}
