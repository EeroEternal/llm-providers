use llm_providers::{get_providers_data, Model as RustModel, Provider as RustProvider};
use pyo3::prelude::*;
use serde_json;

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
            id: m.id.clone(),
            name: m.name.clone(),
            description: m.description.clone(),
            supports_tools: m.supports_tools,
            context_length: m.context_length,
            input_price: m.input_price,
            output_price: m.output_price,
        }
    }
}

/// Represents a Provider configuration.
#[pyclass]
#[derive(Clone)]
pub struct Provider {
    #[pyo3(get)]
    pub label: String,
    #[pyo3(get)]
    pub base_url: String,
    #[pyo3(get)]
    pub models: Vec<Model>,
    #[pyo3(get)]
    pub docs_url: Option<String>,
}

impl From<&RustProvider> for Provider {
    fn from(p: &RustProvider) -> Self {
        Provider {
            label: p.label.clone(),
            base_url: p.base_url.clone(),
            models: p.models.iter().map(Model::from).collect(),
            docs_url: p.docs_url.clone(),
        }
    }
}

/// List all provider IDs
#[pyfunction]
fn list_providers() -> Vec<String> {
    let mut keys: Vec<String> = get_providers_data().keys().cloned().collect();
    keys.sort();
    keys
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
fn get_all_providers() -> std::collections::HashMap<String, Provider> {
    get_providers_data()
        .iter()
        .map(|(k, v)| (k.clone(), Provider::from(v)))
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
    m.add_function(wrap_pyfunction!(get_provider, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_providers, m)?)?;
    m.add_function(wrap_pyfunction!(get_provider_info, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_info, m)?)?;
    m.add_class::<Provider>()?;
    m.add_class::<Model>()?;
    Ok(())
}
