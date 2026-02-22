import json
from datetime import datetime

# Explicit mapping for provider metadata
# Key -> (provider_family, region)
PROVIDER_METADATA = {
    "aliyun": ("aliyun", "cn"),
    "anthropic": ("anthropic", "global"),
    "deepseek": ("deepseek", "cn"),
    "longcat": ("longcat", "cn"),
    "minimax": ("minimax", "cn"),
    "minimax-global": ("minimax", "global"),
    "moonshot": ("moonshot", "cn"),
    "moonshot-global": ("moonshot", "global"),
    "openai": ("openai", "global"),
    "tencent": ("tencent", "cn"),
    "volcengine": ("volcengine", "cn"),
    "zhipu": ("zhipu", "cn"),
    "zhipu-global": ("zhipu", "global"),
}

def migrate():
    # Note: This script assumes the input is the OLD format (map of providers).
    # Since we've already migrated to the new format, running this on the NEW format might break things.
    # This is kept for reference or re-migration from source.
    
    try:
        with open("providers.json", "r") as f:
            data = json.load(f)
    except FileNotFoundError:
        print("providers.json not found.")
        return

    # Check if already migrated
    if "version" in data and "providers" in data:
        print("providers.json already seems to be in the new format. Skipping migration.")
        return

    new_data = {
        "version": "1.0",
        "updated_at": datetime.utcnow().isoformat() + "Z",
        "providers": {}
    }

    for key, provider in data.items():
        # Use explicit mapping, fallback to heuristics only if unknown
        if key in PROVIDER_METADATA:
            family, region = PROVIDER_METADATA[key]
        else:
            print(f"Warning: Unknown provider '{key}', using heuristics.")
            family = key.split("-")[0]
            region = "global" if "global" in key else "cn"

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
