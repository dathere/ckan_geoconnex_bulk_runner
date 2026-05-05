use crate::schema::get_dataset_schema;

pub fn construct_dataset_jsonld_from_metadata(metadata: serde_json::Value) -> serde_json::Value {
    todo!()
}

pub fn validate_dataset_jsonld(jsonld: serde_json::Value) -> bool {
    if let Ok(_) = jsonschema::validate(&get_dataset_schema(), &jsonld) {
        true
    } else {
        false
    }
}
