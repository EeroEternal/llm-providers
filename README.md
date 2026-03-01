<div align="center">
  <h1>LLM Providers</h1>
  <p>
    <strong>A unified source of truth for LLM providers, models, pricing, and capabilities.</strong>
  </p>
  <p>
    Accessible via Rust and Python.
  </p>

  <p>
    <a href="https://github.com/lipish/llm-providers/actions"><img src="https://img.shields.io/github/actions/workflow/status/lipish/llm-providers/ci.yml?branch=main" alt="Build Status"></a>
    <a href="https://crates.io/crates/llm_providers"><img src="https://img.shields.io/crates/v/llm_providers.svg" alt="Crates.io"></a>
    <a href="https://pypi.org/project/llm-providers-list"><img src="https://img.shields.io/pypi/v/llm-providers-list.svg" alt="PyPI"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  </p>
</div>

<div align="center">
  <a href="#installation">Installation</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#usage">Usage</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#supported-providers">Supported Providers</a>
  <span>&nbsp;&nbsp;•&nbsp;&nbsp;</span>
  <a href="#contributing">Contributing</a>
</div>

<br>

## Philosophy

Managing LLM provider configurations across multiple projects and languages is a pain. **LLM Providers** solves this by maintaining a single, curated JSON source of truth for:

- **Model IDs & Names**
- **Context Windows & Max Output Tokens**
- **Pricing (Input/Output)**
- **Capabilities (Tool Support, Vision, etc.)**

This data is **embedded at compile-time** into a Rust crate for zero-latency access and exposed to Python via high-performance bindings.

## Features

- 🚀 **Zero-Latency**: Data is compiled into the binary; no runtime I/O or API calls.
- 🦀 **Rust Native**: Type-safe structs (`Provider`, `Model`) for robust development.
- 🐍 **Python Ready**: Seamless integration via `pip install llm-providers-list`.
- 🔄 **Unified Schema**: Consistent data structure across all providers (OpenAI, Anthropic, DeepSeek, etc.).
- 📦 **Rich Metadata**: Includes pricing, context length, and tool support flags.

## Installation

### Rust

Add this to your `Cargo.toml`:

```toml
[dependencies]
llm_providers = "0.6"
```

### Python

Install via pip (once published) or build from source:

```bash
# From PyPI (Coming Soon)
pip install llm-providers-list

# Build from source (requires maturin)
pip install maturin
maturin develop -m py/Cargo.toml
```

## Usage

### Rust

```rust
use llm_providers::{get_providers_data, list_providers, list_endpoints, get_endpoint, get_model};

fn main() {
    // 1. List all provider family IDs
    let families = list_providers();
    println!("Families: {:?}", families);

    // 2. List all endpoint IDs (for direct configuration)
    let endpoints = list_endpoints();
    println!("Endpoints: {:?}", endpoints);

    // 3. Get endpoint details by ID
    if let Some((family_id, ep)) = get_endpoint("moonshot:global") {
        println!("Family: {}, Base URL: {}, Region: {}", family_id, ep.base_url, ep.region);
    }

    // 4. Get specific model details
    if let Some(model) = get_model("openai", "gpt-4o") {
        println!("Model: {} (Price: ${}/1M input)", model.name, model.input_price);
    }
}
```

### Python

```python
import llm_providers_list

# 1. List all provider families
print(llm_providers_list.list_providers())
# Output: ['aliyun', 'anthropic', 'deepseek', 'openai', ...]

# 2. List all endpoint IDs (for direct configuration)
print(llm_providers_list.list_endpoints())
# Output: ['aliyun:cn', 'anthropic:global', 'moonshot:cn', 'moonshot:global', ...]

# 3. Get endpoint details by ID
family_id, ep = llm_providers_list.get_endpoint("moonshot:global")
print(f"Family: {family_id}, Base URL: {ep.base_url}, Region: {ep.region}")

# 4. Get specific model details
model = llm_providers_list.get_model("openai", "gpt-4o")
print(f"Model: {model.name}, Price: ${model.input_price}/1M tokens")
```

## Advanced Usage

### Filtering Models

You can filter models by region, capabilities (tools), or context length.

#### Rust

```rust
use llm_providers::{filter_models, ModelFilter};

let cn_models = filter_models(ModelFilter {
    region: Some("cn".to_string()),
    supports_tools: Some(true),
    ..Default::default()
});

for (provider_id, model) in cn_models {
    println!("{}: {}", provider_id, model.name);
}
```

#### Python

```python
import llm_providers_list

# Find all models in CN region that support tools
cn_tools_models = llm_providers_list.filter_models(
    region="cn", 
    supports_tools=True
)

for provider_id, model in cn_tools_models:
    print(f"[{provider_id}] {model.name}")
```

### Data Structure

The registry is embedded in the crate as Rust static data (PHF maps, no runtime JSON parsing).

- **Provider Family**: Top-level grouping (e.g., `openai`, `moonshot`). Models are defined once per family.
- **Endpoint**: Regional API entry point under a family. Each endpoint has its own `base_url`, `region`, and `price_currency`.
- **Region**: `cn` (China), `global` (International), `us`, `eu`, etc.
- **Price Currency**: `USD` or `CNY`, defined at the endpoint level.

### Endpoint ID Format (v0.6+)

Endpoint IDs are now **composite** to avoid ambiguity across providers:

- `"{provider_id}:{endpoint_key}"` (recommended)
- Example: `moonshot:global`, `zhipu:cn`

See [docs/providers.md](docs/providers.md) for a full list of endpoint IDs.


## Supported Providers

- **OpenAI** (GPT-4o, GPT-3.5, o1)
- **Anthropic** (Claude 3.5 Sonnet, Haiku, Opus)
- **DeepSeek** (Chat, Reasoner)
- **Aliyun** (Qwen Max, Plus, Turbo)
- **Tencent** (Hunyuan)
- **Moonshot** (Kimi)
  - *Moonshot AI (CN)*
  - *Moonshot AI Global*
- **MiniMax**
  - *MiniMax (CN)*
  - *MiniMax Global*
- **Zhipu** (GLM-4)
  - *BigModel (Zhipu CN)*
  - *Z.ai (Zhipu Global)*
- **Volcengine** (Doubao)
- **LongCat**

## Contributing

Contributions are welcome! To add a new provider or update existing models:

1.  Edit `data/providers.json`.
2.  Run tests to ensure validity (this will run `build.rs` to embed the registry at compile time):
    ```bash
    cargo test
    ```
3.  Submit a Pull Request.

## License

MIT
