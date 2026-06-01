# Endpoint ID Reference

All available Endpoint IDs with their label, region, and base URL. Use these IDs with `get_endpoint()` for direct configuration.

## Global Endpoints

| Endpoint ID | Family | Label | Base URL |
|---|---|---|---|
| `openai:global` | openai | OpenAI | `https://api.openai.com/v1` |
| `anthropic:global` | anthropic | Anthropic | `https://api.anthropic.com` |
| `minimax:global` | minimax | MiniMax Global | `https://api.minimax.io/v1` |
| `moonshot:global` | moonshot | Moonshot AI Global | `https://api.moonshot.ai/v1` |
| `zhipu:global` | zhipu | Z.ai (Zhipu Global) | `https://api.z.ai/api/paas/v4` |

## China (CN) Endpoints

| Endpoint ID | Family | Label | Base URL |
|---|---|---|---|
| `aliyun:cn` | aliyun | Aliyun Bailian | `https://dashscope.aliyuncs.com/compatible-mode/v1` |
| `deepseek:cn` | deepseek | DeepSeek | `https://api.deepseek.com/v1` |
| `minimax:cn` | minimax | MiniMax | `https://api.minimaxi.com/v1` |
| `moonshot:cn` | moonshot | Moonshot AI | `https://api.moonshot.cn/v1` |
| `tencent:cn` | tencent | Tencent Hunyuan | `https://hunyuan.tencentcloudapi.com` |
| `volcengine:cn` | volcengine | Volcengine | `https://ark.cn-beijing.volces.com/api/v3` |
| `zhipu:cn` | zhipu | BigModel (Zhipu CN) | `https://open.bigmodel.cn/api/paas/v4` |
| `longcat:cn` | longcat | LongCat | `https://api.longcat.chat/openai/v1` |

## Usage Example

Use the Endpoint ID to directly access a specific API entry point:

### Rust

```rust
use llm_providers::get_endpoint;

if let Some((family_id, ep)) = get_endpoint("moonshot:global") {
    println!("Family: {}, Base URL: {}", family_id, ep.base_url);
}
```
