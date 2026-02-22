import json
from datetime import datetime

def migrate():
    with open("providers.json", "r") as f:
        old_data = json.load(f)

    new_data = {
        "version": "1.0",
        "updated_at": datetime.utcnow().isoformat() + "Z",
        "providers": {}
    }

    for key, provider in old_data.items():
        # Default values
        family = key.split("-")[0]
        region = "cn"  # Default to CN as many are Chinese providers
        
        # Heuristics for region and family
        if "global" in key:
            region = "global"
            family = key.replace("-global", "")
        elif key in ["openai", "anthropic"]:
            region = "global"
            family = key
        
        # Special cases
        if key == "aliyun":
            family = "aliyun"
            region = "cn"
        elif key == "volcengine":
            family = "volcengine"
            region = "cn"
        
        # Create new provider object
        new_provider = {
            "label": provider.get("label", key),
            "provider_family": family,
            "region": region,
            "base_url": provider.get("base_url", ""),
            "docs_url": provider.get("docs_url"),
            "models": []
        }

        # Migrate models
        for model in provider.get("models", []):
            new_model = {
                "id": model.get("id"),
                "name": model.get("name"),
                "description": model.get("description", ""),
                "supports_tools": model.get("supports_tools", False),
                "context_length": model.get("context_length"),
                "input_price": model.get("input_price", 0.0),
                "output_price": model.get("output_price", 0.0),
                # Infer currency
                "price_currency": "CNY" if region == "cn" else "USD"
            }
            new_provider["models"].append(new_model)

        new_data["providers"][key] = new_provider

    with open("providers.json", "w") as f:
        json.dump(new_data, f, indent=2, ensure_ascii=False)
    
    print("Migration complete!")

if __name__ == "__main__":
    migrate()
