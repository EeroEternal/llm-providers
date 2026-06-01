use crate::{
    Model, canonical_model_id, list_offerings, provider_catalog_priority, registry_updated_at,
    registry_version,
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ModelOffering {
    pub provider_id: &'static str,
    pub endpoint_key: &'static str,
    pub endpoint_id: String,
    pub model: Model,
    pub price_currency: &'static str,
    pub region: &'static str,
    pub base_url: &'static str,
}

#[derive(Debug, Serialize, Clone)]
pub struct ExportRegistry {
    pub registry_version: String,
    pub registry_updated_at: String,
    pub catalog: Vec<CatalogEntry>,
    pub offerings: Vec<ExportOffering>,
}

#[derive(Debug, Serialize, Clone)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: Option<String>,
    pub supports_tools: bool,
    pub supports_vision: bool,
    pub supports_reasoning: bool,
    pub context_length: Option<u64>,
    pub published_at: Option<String>,
    pub deprecated_at: Option<String>,
    pub replacement_id: Option<String>,
    pub primary_provider_id: String,
    pub primary_endpoint_id: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ExportOffering {
    pub provider_id: String,
    pub endpoint_id: String,
    pub model_id: String,
    pub canonical_model_id: String,
    pub price_currency: String,
    pub region: String,
    pub base_url: String,
    pub global_pricing: GlobalPricing,
    pub supports_tools: bool,
    pub supports_vision: bool,
    pub supports_reasoning: bool,
    pub context_length: Option<u64>,
    pub category: Option<String>,
    pub published_at: Option<String>,
    pub deprecated_at: Option<String>,
    pub replacement_id: Option<String>,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub struct GlobalPricing {
    pub prompt: f64,
    pub completion: f64,
    pub cache_read: Option<f64>,
    pub cache_write: Option<f64>,
    pub reasoning: Option<f64>,
}

impl From<&Model> for GlobalPricing {
    fn from(model: &Model) -> Self {
        GlobalPricing {
            prompt: model.input_price,
            completion: model.output_price,
            cache_read: model.cache_read_price,
            cache_write: model.cache_write_price,
            reasoning: model.reasoning_price,
        }
    }
}

pub fn export_pararouter_registry() -> ExportRegistry {
    let offerings = list_offerings();
    ExportRegistry {
        registry_version: registry_version().to_string(),
        registry_updated_at: registry_updated_at().to_string(),
        catalog: build_catalog_from_offerings(&offerings),
        offerings: offerings.iter().map(export_offering).collect(),
    }
}

pub(crate) fn build_catalog_from_offerings(offerings: &[ModelOffering]) -> Vec<CatalogEntry> {
    use std::collections::BTreeMap;

    let mut best: BTreeMap<String, &ModelOffering> = BTreeMap::new();

    for offering in offerings {
        let canonical = canonical_model_id(offering.model.id).to_string();
        best.entry(canonical)
            .and_modify(|current| {
                if catalog_preference(offering, current) == std::cmp::Ordering::Less {
                    *current = offering;
                }
            })
            .or_insert(offering);
    }

    let mut catalog = best
        .into_iter()
        .map(|(canonical_id, offering)| catalog_entry(canonical_id, offering))
        .collect::<Vec<_>>();

    catalog.sort_by(|a, b| a.id.cmp(&b.id));
    catalog
}

fn catalog_preference(candidate: &ModelOffering, current: &&ModelOffering) -> std::cmp::Ordering {
    let candidate_deprecated = candidate.model.is_deprecated();
    let current_deprecated = current.model.is_deprecated();
    if candidate_deprecated != current_deprecated {
        return candidate_deprecated.cmp(&current_deprecated);
    }

    let candidate_priority = provider_catalog_priority(candidate.provider_id);
    let current_priority = provider_catalog_priority(current.provider_id);
    if candidate_priority != current_priority {
        return candidate_priority.cmp(&current_priority);
    }

    candidate
        .provider_id
        .cmp(current.provider_id)
        .then_with(|| candidate.endpoint_id.cmp(&current.endpoint_id))
}

fn catalog_entry(canonical_id: String, offering: &ModelOffering) -> CatalogEntry {
    CatalogEntry {
        id: canonical_id,
        name: offering.model.name.to_string(),
        description: offering.model.description.to_string(),
        category: offering.model.category.map(str::to_string),
        supports_tools: offering.model.supports_tools,
        supports_vision: offering.model.supports_vision,
        supports_reasoning: offering.model.supports_reasoning,
        context_length: offering.model.context_length,
        published_at: offering.model.published_at.map(str::to_string),
        deprecated_at: offering.model.deprecated_at.map(str::to_string),
        replacement_id: offering.model.replacement_id.map(str::to_string),
        primary_provider_id: offering.provider_id.to_string(),
        primary_endpoint_id: offering.endpoint_id.clone(),
    }
}

fn export_offering(offering: &ModelOffering) -> ExportOffering {
    ExportOffering {
        provider_id: offering.provider_id.to_string(),
        endpoint_id: offering.endpoint_id.clone(),
        model_id: offering.model.id.to_string(),
        canonical_model_id: canonical_model_id(offering.model.id).to_string(),
        price_currency: offering.price_currency.to_string(),
        region: offering.region.to_string(),
        base_url: offering.base_url.to_string(),
        global_pricing: GlobalPricing::from(&offering.model),
        supports_tools: offering.model.supports_tools,
        supports_vision: offering.model.supports_vision,
        supports_reasoning: offering.model.supports_reasoning,
        context_length: offering.model.context_length,
        category: offering.model.category.map(str::to_string),
        published_at: offering.model.published_at.map(str::to_string),
        deprecated_at: offering.model.deprecated_at.map(str::to_string),
        replacement_id: offering.model.replacement_id.map(str::to_string),
    }
}
