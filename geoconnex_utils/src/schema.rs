use serde_json::json;

pub fn get_dataset_schema() -> serde_json::Value {
    // Allow for "local" feature
    #[allow(unused_mut)]
    let mut dataset_schema = json!({
        "type": "object",
        "properties": {
            "@context": {"type": ["string", "object"]},
            "@type": {"type": ["string", "array"], "contains": {"anyOf": [{"const": "Dataset"}, {"const": "schema:Dataset"}]}},
            "@id": {"type": "string"},
            "name": {"type": "string"},
            "schema:name": {"type": "string"},
            "provider": {
                "type": "object",
                "properties": {"@type": {"type": "string"}, "name": {"type": "string"}},
            },
            "schema:provider": {
                "type": "object",
                "properties": {"@type": {"type": "string"}, "name": {"type": "string"}},
            },
            "gsp:hasGeometry": {"@type": "object"},
            "about": {
                "type": ["string", "array"],
                "items": {
                    "type": "object",
                    "properties": {"@id": {"type": "string"}, "@type": {"const": "Place"}},
                },
                "minItems": 1
            },
        },
        "anyOf": [
            { "required": ["@context", "@type", "@id", "name", "provider", "about"] },
            { "required": ["@context", "@type", "@id", "name", "provider", "gsp:hasGeometry"] },
            { "required": ["@context", "@type", "@id", "schema:name", "schema:provider", "about"] },
            { "required": ["@context", "@type", "@id", "schema:name", "schema:provider", "gsp:hasGeometry"] },
            // { "required": ["@context", "@type", "@id", "name", "provider"] }
        ]
    });
    // Some JSON-LD for datasets (e.g. sciencebase) do not have about or gsp:hasGeometry yet are still valid as per SHACL shape
    #[cfg(feature = "local")]
    {
        let required_array = dataset_schema
            .get_mut("anyOf")
            .unwrap()
            .as_array_mut()
            .unwrap();
        required_array.insert(
            required_array.len(),
            json!({ "required": ["@context", "@type", "@id", "schema:name", "schema:provider"] }),
        );
    }
    dataset_schema
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
        "anyOf": [
            { "required": ["@context", "@type", "@id", "name", "provider", "geo", "gsp:hasGeometry"] },
            { "required": ["@context", "@type", "@id", "schema:name", "schema:provider", "geo", "gsp:hasGeometry"] },
        ]
    })
}
