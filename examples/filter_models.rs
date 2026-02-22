use llm_providers::{filter_models, ModelFilter};

fn main() {
    // 1. Find all models in CN region
    let cn_models = filter_models(ModelFilter {
        region: Some("cn".to_string()),
        ..Default::default()
    });
    println!("Found {} models in CN region:", cn_models.len());
    for (pid, model) in cn_models.iter().take(5) {
        println!("- [{}] {}", pid, model.name);
    }

    // 2. Find models that support tools
    let tool_models = filter_models(ModelFilter {
        supports_tools: Some(true),
        ..Default::default()
    });
    println!("\nFound {} models supporting tools:", tool_models.len());
    for (pid, model) in tool_models.iter().take(5) {
        println!("- [{}] {}", pid, model.name);
    }
}
