<div align="center">
  <h1>LLM Providers</h1>
  <p>
    <strong>A unified source of truth for LLM providers, models, pricing, and capabilities.</strong>
  </p>
  <p>
    Rust-native registry with a stable ParaRouter export.
  </p>

  <p>
    <a href="https://github.com/lipish/llm-providers/actions"><img src="https://github.com/lipish/llm-providers/ci.yml/badge.svg" alt="Build Status"></a>
    <a href="https://crates.io/crates/llm_providers"><img src="https://img.shields.io/crates/v/llm_providers.svg" alt="Crates.io"></a>
    <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  </p>
</div>

## Features

- Compile-time embedded registry (`data/providers.json`)
- Endpoint-aware pricing with `price_currency` (USD/CNY)
- Extended pricing: prompt/completion/cache_read/cache_write/reasoning
- Global catalog dedupe (`list_catalog_models`) vs full offerings (`list_offerings`)
- Official ParaRouter export CLI and JSON schema

## Installation

```toml
[dependencies]
llm_providers = "0.14"
```

## Usage

```rust
use llm_providers::{
    canonical_model_id, export_pararouter_registry, get_model_for_endpoint,
    list_catalog_models, list_offerings,
};

fn main() {
    // Endpoint-accurate pricing + currency
    let resolved = get_model_for_endpoint("minimax:cn", "MiniMax-M3").unwrap();
    assert_eq!(resolved.price_currency, "CNY");

    // Global catalog (vendor wins over resellers for duplicate ids)
    let catalog = list_catalog_models();
    println!("catalog entries: {}", catalog.len());

    // All deployable provider×endpoint×model tuples
    let offerings = list_offerings();
    println!("offerings: {}", offerings.len());

    // Canonical id helper
    assert_eq!(canonical_model_id("google/gemini-3.5-flash"), "gemini-3.5-flash");

    // Programmatic export
    let export = export_pararouter_registry();
    println!("export version: {}", export.registry_version);
}
```

## ParaRouter Export

```bash
cargo run -- export --format pararouter --output llm_providers_registry.json
```

Export shape is documented in [`schemas/pararouter-export-v1.json`](schemas/pararouter-export-v1.json):

- `catalog[]` — one entry per canonical model id
- `offerings[]` — every `(provider_id, endpoint_id, model_id)` with `price_currency` and `global_pricing`

## Catalog vs Offerings

| Layer | Purpose |
|---|---|
| `catalog` | Global model list keyed by official short id (`gemini-3.5-flash`) |
| `offerings` | Routing/deploy entries including resellers (`tencent`, `volcengine`) |

Reseller/aggregator prefixes like `google/` are stripped by `canonical_model_id()` and must not be used as catalog ids.

## Contributing

Edit `data/providers.json`, then run `cargo test`. See [AGENTS.md](AGENTS.md) and [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
