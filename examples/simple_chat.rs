use llm_connector::{LlmClient, ChatRequest, Message, Role};
use llm_providers::get_providers_data;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. 获取所有 Provider 数据
    let providers = get_providers_data();
    
    // 2. 检查是否有 OpenAI 配置
    // 为了演示方便，我们这里使用 expect，实际代码中应处理 Option
    if let Some(openai) = providers.get("openai") {
        println!("Found Provider: {}", openai.label);
        
        // 3. 选择一个模型 (例如 gpt-4o)
        if let Some(model) = openai.models.iter().find(|m| m.id == "gpt-4o") {
            println!("Selected Model: {} (Context: {:?})", model.name, model.context_length);

            // 4. 从环境变量获取 API Key (请确保已设置)
            if let Ok(api_key) = env::var("OPENAI_API_KEY") {
                // 5. 初始化 LLM Connector
                let client = LlmClient::openai(&api_key)?;

                // 6. 构造请求
                let request = ChatRequest {
                    model: model.id.clone(),
                    messages: vec![
                        Message::text(Role::User, "Hello, who are you?")
                    ],
                    ..Default::default()
                };

                println!("\nSending request to OpenAI...");

                // 7. 发送请求并获取响应
                match client.chat(&request).await {
                    Ok(response) => println!("\nResponse: {}", response.content),
                    Err(e) => eprintln!("\nError sending request: {}", e),
                }
            } else {
                println!("\nOPENAI_API_KEY not set. Skipping API call.");
            }
        } else {
            println!("Model gpt-4o not found.");
        }
    } else {
        println!("OpenAI provider not found.");
    }

    Ok(())
}
