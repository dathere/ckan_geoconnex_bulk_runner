use serde_json::json;

pub fn construct_dataset_jsonld_from_metadata(
    dataset_metadata: serde_json::Value,
) -> serde_json::Value {
    let dataset_id = dataset_metadata.get("id").unwrap().as_str().unwrap();
    let dataset_name = dataset_metadata.get("name").unwrap().as_str().unwrap();
    let organization_name = dataset_metadata
        .get("organization")
        .unwrap()
        .get("title")
        .unwrap();
    // TODO: Align and include Geoconnex PIDs for reference feature categories to extract PIDs from them
    // Then also convert spatial_full FeatureCollection to Multipolygon if needed for gsp:hasGeometry when there are
    // also non-reference feature polygons
    // if let Some(spatial_full) = dataset_metadata.get("spatial_full") {}
    let jsonld = json!({
        "@context": {
            "@vocab": "https://schema.org/",
            "gsp": "http://www.opengis.net/ont/geosparql#",
        },
        "@type": "Dataset",
        // TODO: Customize namespace based on CKAN instance being used
        "@id": format!("https://geoconnex.us/nmwdh/ckan-datasets/{dataset_id}"),
        "name": dataset_name,
        "provider": {
            "@type": "Organization",
            "name": organization_name
        }
    });
    jsonld
}
