use llm_providers::{ModelFilter, filter_offerings, list_catalog_models};

fn main() {
    let catalog = list_catalog_models();
    println!("Catalog models (deduped): {}", catalog.len());

    let cn_offerings = filter_offerings(ModelFilter {
        region: Some("cn".to_string()),
        ..Default::default()
    });
    println!("Active CN offerings: {}", cn_offerings.len());
    for offering in cn_offerings.iter().take(5) {
        println!(
            "- [{}] {} ({})",
            offering.endpoint_id, offering.model.name, offering.price_currency
        );
    }
}
