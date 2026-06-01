# Agent Guide: Adding or Updating Models

This repository maintains a single curated registry at `data/providers.json`. All Rust/Python APIs read from this file at compile time. When adding a model, **only edit `data/providers.json`** unless you are also cutting a release.

## Registry Layout

```json
{
  "version": "3.0",
  "updated_at": "2026-06-01T00:00:00.000000Z",
  "providers": {
    "<provider_id>": { ... }
  }
}
```

- Bump `updated_at` to the current UTC timestamp on every registry change.
- `provider_id` is the stable family key (e.g. `minimax`, `deepseek`, `zhipu`).

## Provider Structure

Each provider family contains:

| Field | Required | Purpose |
|---|---|---|
| `label` | yes | Human-readable provider name |
| `endpoints` | yes | Regional API entry points |
| `models` | usually yes | Provider-level model list (fallback / union) |
| `endpoint_models` | optional | Per-endpoint model lists with region-specific pricing |

### Endpoints (`cn`, `global`, …)

Each endpoint key maps to:

```json
{
  "label": "MiniMax",
  "region": "cn",
  "base_url": "https://api.minimaxi.com/v1",
  "price_currency": "CNY",
  "docs_url": "https://platform.minimaxi.com/docs/guides/models-intro"
}
```

Rules:

- **`region`**: use `cn` for China, `global` for international. Do not encode region in `label`.
- **`base_url`**: official OpenAI-compatible base URL for that region (no trailing path beyond what the vendor documents).
- **`price_currency`**: `CNY` for `cn`, `USD` for `global`.
- **`docs_url`**: prefer the vendor’s model overview or pricing page.

Composite endpoint IDs used by the API:

```
{provider_id}:{endpoint_key}
```

Examples: `minimax:cn`, `minimax:global`, `moonshot:global`.

## Model Fields

Every model entry must include:

| Field | Type | Notes |
|---|---|---|
| `id` | string | **Must match the vendor API model ID exactly** (official short id; no `google/` prefixes) |
| `name` | string | Display name |
| `description` | string | Capabilities, context, deprecation notes |
| `supports_tools` | bool | `true` for chat/LLM with function calling; `false` for TTS/video/image/music |
| `supports_vision` | bool (optional) | Default `false` |
| `supports_reasoning` | bool (optional) | Default `false` |
| `context_length` | number or null | Context window in tokens; use official vendor value |
| `input_price` | number | Price per **1M input tokens** (maps to export `global_pricing.prompt`) |
| `output_price` | number | Price per **1M output tokens** (maps to export `global_pricing.completion`) |
| `cache_read_price` | number or null | Cache read $/1M; omit or `null` if unknown |
| `cache_write_price` | number or null | Cache write $/1M; omit or `null` if unknown |
| `reasoning_price` | number or null | Reasoning/thinking $/1M; omit or `null` if unknown |
| `category` | string (optional) | e.g. `Coding`, `Reasoning` |
| `published_at` | string (optional) | `YYYY-MM-DD` release date; required for new non-deprecated models (build warns if missing) |
| `deprecated_at` | string (optional) | `YYYY-MM-DD` when the model id stops being recommended |
| `replacement_id` | string (optional) | Canonical replacement model id |

### Pricing Rules

1. Record **list / standard pay-as-you-go** prices, not temporary promos, unless the whole provider currently has no stable list price.
2. Prefer **cache-miss** tier when the vendor publishes tiered input pricing (e.g. MiniMax ≤512K standard tier).
3. Use the endpoint’s currency:
   - `minimax:cn` → CNY (e.g. M3: `4.2` / `16.8`)
   - `minimax:global` → USD (e.g. M3: `0.6` / `2.4`)
4. Use `0.0` / `0.0` for limited-time free or non-token billing (TTS, video, image).
5. Prices must be finite numbers (`build.rs` rejects NaN/Inf).

### Model ID Conventions

Follow the vendor exactly:

- MiniMax: `MiniMax-M3`, `MiniMax-M2.7-highspeed`
- DeepSeek: `deepseek-v4-flash`, `deepseek-v4-pro`
- Zhipu: `glm-5.1`
- Moonshot: `kimi-k2.6`
- Xiaomi: `mimo-v2.5-pro`

Do not invent aliases unless the vendor documents them as official API IDs.

## `models` vs `endpoint_models`

### Provider-level `models`

- Used by `list_models(provider_id)` and `get_model(provider_id, model_id)`.
- Should list **all models** for the family.
- When CN and global prices differ, store a reasonable default (usually CN values if the provider is China-first, or the primary documented region).

### `endpoint_models` (recommended when CN ≠ global)

Use when the same model ID has **different pricing or availability** per region.

```json
"endpoint_models": {
  "cn": [ /* models with CNY prices */ ],
  "global": [ /* models with USD prices */ ]
}
```

API behavior:

- `get_model_for_endpoint("minimax:global", "MiniMax-M3")` → global pricing
- `get_model_for_endpoint("minimax:cn", "MiniMax-M3")` → CN pricing
- If `endpoint_models` exists for an endpoint, it takes precedence; otherwise fall back to `models`.

**Important:** If you add `endpoint_models` for an endpoint key, that array must be non-empty. You do not need to duplicate every model in `endpoint_models` on day one—start with models whose regional pricing differs (text/LLM models). Speech/video/image models can remain only in `models` until regional pricing is needed.

If `models` is empty but `endpoint_models` is present, `build.rs` auto-builds `models` as the union of all endpoint model lists (deduped by `id`).

## Checklist: Add a New Model

Use **MiniMax M3** as the reference workflow.

### 1. Gather official data

From vendor docs (model page + pricing page):

- API model ID
- Context window and max output (put context in `context_length`; mention max output in `description` if useful)
- Capabilities: tools, vision, thinking, web search
- CN and global pricing (if both exist)
- Release date

Example sources:

- Models: https://platform.minimax.io/docs/guides/models-intro
- Pricing: https://platform.minimax.io/docs/guides/pricing-paygo
- Product page: https://www.minimax.io/models/text/m3

### 2. Edit `data/providers.json`

Under the correct `provider_id`:

1. Add the model near the top of `models` (newest/flagship first is the project convention).
2. If CN/global prices differ, also add entries under `endpoint_models.cn` and `endpoint_models.global`.
3. Update `description` for related legacy models if the vendor announces deprecation/routing (e.g. “auto-routes to … on 2026-06-30”).
4. Bump `updated_at`.

Example (MiniMax M3):

```json
{
  "id": "MiniMax-M3",
  "name": "MiniMax M3",
  "description": "Frontier multimodal coding and agentic model with 1M MSA context, native vision, tool use, and long-horizon task execution",
  "supports_tools": true,
  "context_length": 1000000,
  "input_price": 4.2,
  "output_price": 16.8,
  "published_at": "2026-06-01"
}
```

With regional pricing:

```json
"endpoint_models": {
  "cn": [
    { "id": "MiniMax-M3", "input_price": 4.2, "output_price": 16.8, ... }
  ],
  "global": [
    { "id": "MiniMax-M3", "input_price": 0.6, "output_price": 2.4, ... }
  ]
}
```

### 3. Reseller / aggregator providers

Some models appear on platforms that resell third-party models. Add them under the **reseller’s** provider if they expose a distinct API endpoint and model ID:

| Provider | Examples |
|---|---|
| `tencent` | `deepseek-v4-flash`, `kimi-k2.6`, `minimax-m2.7`, `glm-5.1` |
| `volcengine` | `deepseek-v4-flash`, `kimi-k2.6`, `MiniMax-M2.7`, `glm-5.1` |

Use the **reseller’s model ID and CNY pricing**, not the upstream vendor’s ID, when they differ.

Duplicate ids across vendor + reseller families are intentional. Global catalog dedupe prefers the **vendor family** over resellers (`tencent`, `volcengine`, aggregators). Use `list_offerings()` for all deployable tuples; use `list_catalog_models()` for a flat canonical catalog.

### 4. Validate

```bash
cargo test
cargo run -- export --format pararouter --output /tmp/registry.json
```

This runs `build.rs`, validates JSON, embeds the registry, and executes unit tests. Fix any panic from invalid JSON, empty endpoints, or empty `endpoint_models` arrays.

Optional:

```bash
cargo fmt --all -- --check
```

### 5. Release (when asked)

Only when explicitly requested:

1. Bump `version` in root `Cargo.toml` (semver).
2. Commit with conventional message, e.g. `feat: add MiniMax M3 model`.
3. Tag `vX.Y.Z`, push to GitHub.
4. Publish: `cargo publish`

## Common Mistakes

- Using display names instead of API IDs (`MiniMax M3` vs `MiniMax-M3`).
- Putting USD prices on `cn` endpoints or vice versa.
- Forgetting `endpoint_models.global` when CN/global text pricing differs.
- Setting `supports_tools: true` on TTS/video/image models.
- Leaving stale `context_length` (e.g. 32K when vendor documents 256K or 1M).
- Not noting deprecation/auto-routing in `description` for legacy model IDs.
- Not setting structured `deprecated_at` / `replacement_id` when retiring model ids.
- Forgetting to bump `updated_at`.

## Quick API Reference

```rust
use llm_providers::{
    canonical_model_id, export_pararouter_registry, get_endpoint, get_model_for_endpoint,
    list_catalog_models, list_offerings,
};

// Endpoint base URL + currency + regional pricing
let resolved = get_model_for_endpoint("minimax:global", "MiniMax-M3").unwrap();
// resolved.price_currency, resolved.model.input_price

// Global catalog (deduped by canonical id; vendor beats reseller)
let catalog = list_catalog_models();

// All offerings for routing / Hub sync
let offerings = list_offerings();

// ParaRouter export
let export = export_pararouter_registry();

// Strip aggregator prefixes
assert_eq!(canonical_model_id("google/gemini-3.5-flash"), "gemini-3.5-flash");
```

Export schema: `schemas/pararouter-export-v1.json`.
