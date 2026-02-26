<div align="center">
  <h1>LLM Providers (Python)</h1>
  <p>
    <strong>A unified source of truth for LLM providers, models, pricing, and capabilities.</strong>
  </p>

  <p>
    <a href="https://github.com/lipish/llm-providers/actions"><img src="https://img.shields.io/github/actions/workflow/status/lipish/llm-providers/ci.yml?branch=main" alt="Build Status"></a>
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

## Overview

Python bindings for the [LLM Providers](https://github.com/lipish/llm-providers) registry. Provides zero-latency access to a curated dataset of LLM providers, models, pricing, and capabilities — powered by Rust via PyO3.

The registry is embedded in the native extension at build time from `data/providers.json` via `build.rs` (no runtime JSON parsing).

## Features

- 🚀 **Zero-Latency**: Data is compiled into the binary; no runtime I/O or API calls.
- 🐍 **Pythonic API**: Simple functions and typed objects.
- 🔄 **Unified Schema**: Consistent data structure across all providers (OpenAI, Anthropic, DeepSeek, etc.).
- 📦 **Rich Metadata**: Includes pricing, context length, and tool support flags.

## Installation

```bash
pip install llm-providers-list
```

## Usage

```python
import llm_providers_list

# 1. List all provider families
print(llm_providers_list.list_providers())
# Output: ['aliyun', 'anthropic', 'deepseek', 'openai', ...]

# 2. List all endpoint IDs (for direct configuration)
print(llm_providers_list.list_endpoints())
# Output: ['aliyun', 'anthropic', 'moonshot', 'moonshot_global', ...]

# 3. Get endpoint details by ID
family_id, ep = llm_providers_list.get_endpoint("moonshot_global")
print(f"Family: {family_id}, Base URL: {ep.base_url}, Region: {ep.region}")

# 4. Get specific model details
model = llm_providers_list.get_model("openai", "gpt-4o")
print(f"Model: {model.name}, Price: ${model.input_price}/1M tokens")
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

Contributions are welcome! See the [main repository](https://github.com/lipish/llm-providers) for details.

## License

MIT
