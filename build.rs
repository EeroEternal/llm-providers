use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegistryFile {
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    registry_version: Option<String>,

    updated_at: String,

    providers: BTreeMap<String, ProviderFamily>,
}

#[derive(Debug, Deserialize)]
struct ProviderFamily {
    label: String,
    endpoints: BTreeMap<String, EndpointJson>,
    models: Vec<ModelJson>,
}

#[derive(Debug, Deserialize)]
struct EndpointJson {
    label: String,
    region: String,
    base_url: String,
    price_currency: String,
    #[serde(default)]
    docs_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModelJson {
    id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    supports_tools: bool,
    #[serde(default)]
    context_length: Option<u64>,
    input_price: f64,
    output_price: f64,
}

fn rust_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

fn rust_opt_str(v: &Option<String>) -> String {
    match v {
        Some(s) => format!("Some({})", rust_str(s)),
        None => "None".to_string(),
    }
}

fn rust_f64(v: f64) -> String {
    if v.is_finite() {
        if v.fract() == 0.0 {
            format!("{:.1}", v)
        } else {
            // Keep it unambiguous as a float literal
            let s = format!("{}", v);
            if s.contains('.') || s.contains('e') || s.contains('E') {
                s
            } else {
                format!("{}.0", s)
            }
        }
    } else {
        panic!("price must be finite, got {v}");
    }
}

fn rust_ident_upper(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_uppercase());
        } else {
            out.push('_');
        }
    }
    while out.contains("__") {
        out = out.replace("__", "_");
    }
    out = out.trim_matches('_').to_string();
    if out.is_empty() {
        out = "PROVIDER".to_string();
    }
    if out.chars().next().unwrap().is_ascii_digit() {
        out = format!("P_{out}");
    }
    out
}

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let input_path = Path::new(&manifest_dir).join("data").join("providers.json");

    println!("cargo:rerun-if-changed={}", input_path.display());
    println!("cargo:rerun-if-changed=build.rs");

    let content = fs::read_to_string(&input_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", input_path.display()));

    let mut reg: RegistryFile =
        serde_json::from_str(&content).unwrap_or_else(|e| panic!("invalid providers.json: {e}"));

    // Normalize
    let version = reg
        .registry_version
        .take()
        .or(reg.version.take())
        .unwrap_or_else(|| "2.0".to_string());

    for (pid, p) in reg.providers.iter_mut() {
        if p.endpoints.is_empty() {
            panic!("provider {pid}: endpoints must be non-empty");
        }
        if p.models.is_empty() {
            panic!("provider {pid}: models must be non-empty");
        }
        p.models.sort_by(|a, b| a.id.cmp(&b.id));
    }

    let mut out = String::new();
    out.push_str(&format!(
        "pub static REGISTRY_VERSION: &str = {};\n",
        rust_str(&version)
    ));
    out.push_str(&format!(
        "pub static REGISTRY_UPDATED_AT: &str = {};\n\n",
        rust_str(&reg.updated_at)
    ));

    for (provider_id, provider) in reg.providers.iter() {
        let prefix = rust_ident_upper(provider_id);

        out.push_str(&format!(
            "pub static {prefix}_ENDPOINTS: phf::Map<&'static str, Endpoint> = phf::phf_map! {{\n"
        ));
        for (endpoint_id, ep) in provider.endpoints.iter() {
            out.push_str(&format!(
                "    {} => Endpoint {{ label: {}, region: {}, base_url: {}, price_currency: {}, docs_url: {} }},\n",
                rust_str(endpoint_id),
                rust_str(&ep.label),
                rust_str(&ep.region),
                rust_str(&ep.base_url),
                rust_str(&ep.price_currency),
                rust_opt_str(&ep.docs_url),
            ));
        }
        out.push_str("};\n\n");

        out.push_str(&format!("pub static {prefix}_MODELS: &[Model] = &[\n"));
        for m in provider.models.iter() {
            let desc = m.description.clone().unwrap_or_default();
            out.push_str(&format!(
                "    Model {{ id: {}, name: {}, description: {}, supports_tools: {}, context_length: {}, input_price: {}, output_price: {} }},\n",
                rust_str(&m.id),
                rust_str(&m.name),
                rust_str(&desc),
                if m.supports_tools { "true" } else { "false" },
                match m.context_length {
                    Some(v) => format!("Some({v})"),
                    None => "None".to_string(),
                },
                rust_f64(m.input_price),
                rust_f64(m.output_price),
            ));
        }
        out.push_str("];\n\n");
    }

    out.push_str("pub static PROVIDERS: phf::Map<&'static str, Provider> = phf::phf_map! {\n");
    for (provider_id, provider) in reg.providers.iter() {
        let prefix = rust_ident_upper(provider_id);
        out.push_str(&format!(
            "    {} => Provider {{ label: {}, endpoints: &{prefix}_ENDPOINTS, models: {prefix}_MODELS }},\n",
            rust_str(provider_id),
            rust_str(&provider.label),
        ));
    }
    out.push_str("};\n");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_path = Path::new(&out_dir).join("registry_generated.rs");
    fs::write(&out_path, out).unwrap_or_else(|e| panic!("failed to write {}: {e}", out_path.display()));
}
