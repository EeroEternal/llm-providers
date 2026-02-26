use llm_providers::{
    get_providers_data,
    Model as RustModel,
    Provider as RustProvider,
    Endpoint as RustEndpoint,
    ModelFilter,
};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Represents a Model configuration.
#[pyclass]
#[derive(Clone)]
pub struct Model {
    #[pyo3(get)]
    pub id: String,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub supports_tools: bool,
    #[pyo3(get)]
    pub context_length: Option<u64>,
    #[pyo3(get)]
    pub input_price: f64,
    #[pyo3(get)]
    pub output_price: f64,
}

impl From<&RustModel> for Model {
    fn from(m: &RustModel) -> Self {
        Model {
            id: m.id.to_string(),
            name: m.name.to_string(),
            description: m.description.to_string(),
            supports_tools: m.supports_tools,
            context_length: m.context_length,
            input_price: m.input_price,
            output_price: m.output_price,
        }
    }
}

/// Represents an API endpoint (regional entry point).
#[pyclass]
#[derive(Clone)]
pub struct Endpoint {
    #[pyo3(get)]
    pub label: String,
    #[pyo3(get)]
    pub region: String,
    #[pyo3(get)]
    pub base_url: String,
    #[pyo3(get)]
    pub price_currency: String,
    #[pyo3(get)]
    pub docs_url: Option<String>,
}

impl From<&RustEndpoint> for Endpoint {
    fn from(ep: &RustEndpoint) -> Self {
        Endpoint {
            label: ep.label.to_string(),
            region: ep.region.to_string(),
            base_url: ep.base_url.to_string(),
            price_currency: ep.price_currency.to_string(),
            docs_url: ep.docs_url.map(|s| s.to_string()),
        }
    }
}

/// Represents a Provider family with endpoints and models.
#[pyclass]
#[derive(Clone)]
pub struct Provider {
    #[pyo3(get)]
    pub label: String,
    #[pyo3(get)]
    pub endpoints: HashMap<String, Endpoint>,
    #[pyo3(get)]
    pub models: Vec<Model>,
}

impl From<&RustProvider> for Provider {
    fn from(p: &RustProvider) -> Self {
        Provider {
            label: p.label.to_string(),
            endpoints: p
                .endpoints
                .entries()
                .map(|(k, v)| (k.to_string(), Endpoint::from(v)))
                .collect(),
            models: p.models.iter().map(Model::from).collect(),
        }
    }
}

/// List all provider family IDs
#[pyfunction]
fn list_providers() -> Vec<String> {
    llm_providers::list_providers()
}

/// List all endpoint IDs
#[pyfunction]
fn list_endpoints() -> Vec<String> {
    llm_providers::list_endpoints()
}

/// Get endpoint by ID, returns (family_id, Endpoint)
#[pyfunction]
fn get_endpoint(endpoint_id: &str) -> PyResult<(String, Endpoint)> {
    llm_providers::get_endpoint(endpoint_id)
        .map(|(fid, ep)| (fid.to_string(), Endpoint::from(ep)))
        .ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Endpoint not found")
        })
}

/// List all model IDs for a specific provider
#[pyfunction]
fn list_models(provider_id: &str) -> PyResult<Vec<String>> {
    llm_providers::list_models(provider_id).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err("Provider not found")
    })
}

/// Get detailed information for a specific model
#[pyfunction]
fn get_model(provider_id: &str, model_id: &str) -> PyResult<Model> {
    llm_providers::get_model(provider_id, model_id)
        .map(|m| Model::from(&m))
        .ok_or_else(|| {
            pyo3::exceptions::PyValueError::new_err("Model not found")
        })
}

/// Filter models based on criteria.
/// Returns a list of tuples (provider_id, Model).
#[pyfunction]
#[pyo3(signature = (provider_id=None, region=None, supports_tools=None, min_context_length=None))]
fn filter_models(
    provider_id: Option<String>,
    region: Option<String>,
    supports_tools: Option<bool>,
    min_context_length: Option<u64>,
) -> Vec<(String, Model)> {
    let filter = ModelFilter {
        provider_id,
        region,
        supports_tools,
        min_context_length,
    };
    
    llm_providers::filter_models(filter)
        .into_iter()
        .map(|(pid, m)| (pid, Model::from(&m)))
        .collect()
}

/// Get detailed information for a specific provider as a Provider object
#[pyfunction]
fn get_provider(provider_id: &str) -> PyResult<Provider> {
    let providers = get_providers_data();
    if let Some(p) = providers.get(provider_id) {
        Ok(Provider::from(p))
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(
            "Provider not found",
        ))
    }
}

/// Get all providers as a dictionary of Provider objects
#[pyfunction]
fn get_all_providers() -> HashMap<String, Provider> {
    get_providers_data()
        .entries()
        .map(|(k, v)| (k.to_string(), Provider::from(v)))
        .collect()
}

/// Get detailed information for a specific provider as a JSON string
#[pyfunction]
fn get_provider_info(provider_id: &str) -> PyResult<String> {
    let providers = get_providers_data();
    if let Some(p) = providers.get(provider_id) {
        serde_json::to_string(p).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    } else {
        Err(pyo3::exceptions::PyValueError::new_err(
            "Provider not found",
        ))
    }
}

/// Get all information as a JSON string
#[pyfunction]
fn get_all_info() -> PyResult<String> {
    serde_json::to_string(get_providers_data())
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
}

/// A Python module implemented in Rust.
#[pymodule]
fn llm_providers_list(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(list_providers, m)?)?;
    m.add_function(wrap_pyfunction!(list_endpoints, m)?)?;
    m.add_function(wrap_pyfunction!(get_endpoint, m)?)?;
    m.add_function(wrap_pyfunction!(list_models, m)?)?;
    m.add_function(wrap_pyfunction!(get_model, m)?)?;
    m.add_function(wrap_pyfunction!(filter_models, m)?)?;
    m.add_function(wrap_pyfunction!(get_provider, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_providers, m)?)?;
    m.add_function(wrap_pyfunction!(get_provider_info, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_info, m)?)?;
    m.add_class::<Provider>()?;
    m.add_class::<Endpoint>()?;
    m.add_class::<Model>()?;
    Ok(())
}
