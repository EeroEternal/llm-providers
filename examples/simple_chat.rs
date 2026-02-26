use llm_connector::{ChatRequest, LlmClient, Message, Role};
use llm_providers::get_providers_data;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Get all provider data
    let providers = get_providers_data();

    // 2. Check whether OpenAI is present
    if let Some(openai) = providers.get("openai") {
        let ep = openai
            .endpoints
            .get("openai")
            .expect("openai endpoint not found");
        println!(
            "Found Provider: {} (Base URL: {}, Region: {})",
            openai.label, ep.base_url, ep.region
        );

        // 3. Pick a model (e.g. gpt-4o)
        if let Some(model) = openai.models.iter().find(|m| m.id == "gpt-4o") {
            println!(
                "Selected Model: {} (Context: {:?}, Currency: {})",
                model.name, model.context_length, ep.price_currency
            );

            // 4. Read API key from environment variables (make sure it's set)
            if let Ok(api_key) = env::var("OPENAI_API_KEY") {
                // 5. Initialize LLM Connector
                let client = LlmClient::openai(&api_key)?;

                // 6. Build request
                let request = ChatRequest {
                    model: model.id.to_string(),
                    messages: vec![Message::text(Role::User, "Hello, who are you?")],
                    ..Default::default()
                };

                println!("\nSending request to OpenAI...");

                // 7. Send request and get response
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
