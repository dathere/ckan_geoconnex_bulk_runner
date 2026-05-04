use serde_json::json;

async fn construct_jsonld(metadata: serde_json::Value) -> serde_json::Value {}

async fn get_dataset_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "@context": {"type": ["string", "object"]},
            "@type": {"const": "Dataset"}
        },
        "required": []
    })
}
