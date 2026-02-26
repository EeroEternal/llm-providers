# Endpoint ID Reference

All available Endpoint IDs with their label, region, and base URL. Use these IDs with `get_endpoint()` for direct configuration.

## Global Endpoints

| Endpoint ID | Family | Label | Base URL |
|---|---|---|---|
| `openai` | openai | OpenAI | `https://api.openai.com/v1` |
| `anthropic` | anthropic | Anthropic | `https://api.anthropic.com` |
| `minimax_global` | minimax | MiniMax Global | `https://api.minimax.io/v1` |
| `moonshot_global` | moonshot | Moonshot AI Global | `https://api.moonshot.ai/v1` |
| `zhipu_global` | zhipu | Z.ai (Zhipu Global) | `https://api.z.ai/api/paas/v4` |

## China (CN) Endpoints

| Endpoint ID | Family | Label | Base URL |
|---|---|---|---|
| `aliyun` | aliyun | Aliyun Bailian | `https://dashscope.aliyuncs.com/compatible-mode/v1` |
| `deepseek` | deepseek | DeepSeek | `https://api.deepseek.com/v1` |
| `minimax` | minimax | MiniMax | `https://api.minimaxi.com/v1` |
| `moonshot` | moonshot | Moonshot AI | `https://api.moonshot.cn/v1` |
| `tencent` | tencent | Tencent Hunyuan | `https://hunyuan.tencentcloudapi.com` |
| `volcengine` | volcengine | Volcengine | `https://ark.cn-beijing.volces.com/api/v3` |
| `zhipu` | zhipu | BigModel (Zhipu CN) | `https://open.bigmodel.cn/api/paas/v4` |
| `longcat` | longcat | LongCat | `https://api.longcat.chat/openai/v1` |

## Usage Example

Use the Endpoint ID to directly access a specific API entry point:

### Python

```python
import llm_providers_list

family_id, ep = llm_providers_list.get_endpoint("moonshot_global")
print(f"Family: {family_id}")
print(f"Base URL: {ep.base_url}")
print(f"Region: {ep.region}")
```

### Rust

```rust
use llm_providers::get_endpoint;

if let Some((family_id, ep)) = get_endpoint("moonshot_global") {
    println!("Family: {}, Base URL: {}", family_id, ep.base_url);
}
```
