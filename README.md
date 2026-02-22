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
llm_providers = { git = "https://github.com/lipish/llm-providers.git" }
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
use llm_providers::get_providers_data;

fn main() {
    let providers = get_providers_data();

    if let Some(openai) = providers.get("openai") {
        println!("Provider: {}", openai.label);
        println!("Base URL: {}", openai.base_url);

        for model in &openai.models {
            println!("- {} (Context: {:?})", model.name, model.context_length);
        }
    }
}
```

### Python

```python
import llm_providers_list
import json

# List all supported providers
print(llm_providers_list.list_providers())
# Output: ['aliyun', 'anthropic', 'deepseek', 'openai', ...]

# Get provider object (Rich Type)
openai = llm_providers_list.get_provider("openai")
print(f"Label: {openai.label}")
print(f"Base URL: {openai.base_url}")

for model in openai.models:
    print(f"Model: {model.name}, Price: ${model.input_price}/1M tokens")

# Get raw JSON info
info = llm_providers_list.get_provider_info("anthropic")
print(info)
```

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

1.  Edit `providers.json` in the root directory.
2.  Run tests to ensure validity:
    ```bash
    cargo test
    ```
3.  Submit a Pull Request.

## License

MIT
