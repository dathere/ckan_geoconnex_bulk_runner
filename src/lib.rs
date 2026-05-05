use serde_json::json;

pub fn get_dataset_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "@context": {"type": ["string", "object"]},
            "@type": {"type": ["string", "array"], "contains": {"const": "Dataset"}},
            "@id": {"type": "string"},
            "name": {"type": "string"},
            "provider": {
                "type": "object",
                "properties": {"@type": {"type": "string"}, "name": {"type": "string"}},
            },
            "about": {
                "type": ["string", "array"],
                "items": {
                    "type": "object",
                    "properties": {"@id": {"type": "string"}, "@type": {"const": "Place"}},
                },
                "minItems": 1
            },
        },
        "required": ["@context", "@type", "@id", "name", "provider", "about"]
    })
}

pub fn get_location_schema() -> serde_json::Value {
    json!({
        "type": "object",
        "properties": {
            "@context": {"type": ["string", "object"]},
            "@type": {"type": ["string", "array"], "contains": {"const": "Place"}},
            "@id": {"type": "string"},
            "name": {"type": "string"},
            "provider": {
                "type": "object",
                "properties": {"@type": {"type": "string"}, "name": {"type": "string"}},
            },
            "geo": {"type": "object"},
            "gsp:hasGeometry": {"type": "object"}
        },
        "required": ["@context", "@type", "@id", "name", "provider", "geo", "gsp:hasGeometry"]
    })
}
