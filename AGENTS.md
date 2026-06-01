# Agent Guide: Adding or Updating Models

This repository maintains a single curated registry at `data/providers.json`. All Rust/Python APIs read from this file at compile time. When adding a model, **only edit `data/providers.json`** unless you are also cutting a release.

## Registry Layout

```json
{
  "version": "2.0",
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
| `id` | string | **Must match the vendor API model ID exactly** (case-sensitive) |
| `name` | string | Display name |
| `description` | string | Capabilities, context, deprecation notes |
| `supports_tools` | bool | `true` for chat/LLM with function calling; `false` for TTS/video/image/music |
| `context_length` | number or null | Context window in tokens; use official vendor value |
| `input_price` | number | Price per **1M input tokens** |
| `output_price` | number | Price per **1M output tokens** |
| `published_at` | string (optional) | `YYYY-MM-DD` release date for new models |

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

### 4. Validate

```bash
cargo test
```

This runs `build.rs`, validates JSON, embeds the registry, and executes unit tests. Fix any panic from invalid JSON, empty endpoints, or empty `endpoint_models` arrays.

Optional:

```bash
cargo fmt --all -- --check
```

### 5. Release (when asked)

Only when explicitly requested:

1. Bump `version` in root `Cargo.toml` (semver).
2. Update `llm_providers` path/version in `py/Cargo.toml` if needed.
3. Commit with conventional message, e.g. `feat: add MiniMax M3 model`.
4. Tag `vX.Y.Z`, push to GitHub.
5. Publish: `cargo publish`

## Common Mistakes

- Using display names instead of API IDs (`MiniMax M3` vs `MiniMax-M3`).
- Putting USD prices on `cn` endpoints or vice versa.
- Forgetting `endpoint_models.global` when CN/global text pricing differs.
- Setting `supports_tools: true` on TTS/video/image models.
- Leaving stale `context_length` (e.g. 32K when vendor documents 256K or 1M).
- Not noting deprecation/auto-routing in `description` for legacy model IDs.
- Forgetting to bump `updated_at`.

## Quick API Reference

```rust
use llm_providers::{get_endpoint, get_model, get_model_for_endpoint};

// Endpoint base URL + currency
let (_, ep) = get_endpoint("minimax:global").unwrap();
// ep.base_url, ep.price_currency

// Provider-level model (fallback list)
let model = get_model("minimax", "MiniMax-M3").unwrap();

// Region-accurate pricing
let model = get_model_for_endpoint("minimax:global", "MiniMax-M3").unwrap();
```

```python
import llm_providers_list

family, ep = llm_providers_list.get_endpoint("minimax:cn")
model = llm_providers_list.get_model_for_endpoint("minimax:global", "MiniMax-M3")
```
