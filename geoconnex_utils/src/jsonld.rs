use anyhow::{Result, bail};
use serde_json::json;

pub fn construct_dataset_jsonld_from_metadata(
    dataset_metadata: serde_json::Value,
) -> Result<serde_json::Value> {
    let dataset_id = dataset_metadata.get("id").unwrap().as_str().unwrap();
    eprintln!("Attempting to construct JSON-LD for dataset {dataset_id}");
    let dataset_title = dataset_metadata.get("title").unwrap().as_str().unwrap();
    let organization_name = dataset_metadata
        .get("organization")
        .unwrap()
        .get("title")
        .unwrap();
    // TODO: Align and include Geoconnex PIDs for reference feature categories to extract PIDs from them
    // Then also convert spatial_full FeatureCollection to Multipolygon if needed for gsp:hasGeometry when there are
    // also non-reference feature polygons
    let mut about = vec![];
    if let Some(spatial_full) = dataset_metadata.get("spatial_full") {
        let Some(spatial_full_str) = spatial_full.as_str() else {
            bail!("Could not parse spatial_full as string.");
        };
        if !spatial_full_str.is_empty() {
            let Ok(spatial_full_json) = serde_json::from_str::<serde_json::Value>(spatial_full_str)
            else {
                bail!(
                    "Error while attempting to deserialize spatial_full string to serde_json::Value."
                );
            };
            let Some(features_value) = spatial_full_json.get("features") else {
                bail!("Error while attempting to get value of features from spatial_full GeoJSON.");
            };
            let Some(features) = features_value.as_array() else {
                bail!(
                    "Error while attempting to take features value as array from spatial_full GeoJSON."
                );
            };
            for feature in features {
                let Some(properties) = feature.get("properties") else {
                    bail!(
                        "Error while attempting to get properties from features from spatial_full GeoJSON."
                    );
                };
                if let Some(pid) = properties.get("pid") {
                    let Some(pid_string) = pid.as_str() else {
                        bail!("Error while attempting to convert PID as str from &Value.");
                    };
                    about.push(json!({
                        "@id": pid_string,
                        "@type": "Place"
                    }));
                }
            }
        }
    }
    let mut jsonld = json!({
        "@context": {
            "@vocab": "https://schema.org/",
            "gsp": "http://www.opengis.net/ont/geosparql#",
        },
        "@type": "Dataset",
        // TODO: Customize namespace based on CKAN instance being used
        "@id": format!("https://geoconnex.us/ckan/nmwdh/{dataset_id}"),
        "name": dataset_title,
        "provider": {
            "@type": "Organization",
            "name": organization_name
        },
        // TODO: Customize CKAN instance URL based on CKAN instance being used
        "url": format!("https://catalog.newmexicowaterdata.org/dataset/{dataset_id}")
    });
    let jsonld_map = jsonld.as_object_mut().unwrap();
    if about.len() > 0 {
        jsonld_map.insert("about".to_string(), serde_json::to_value(about).unwrap());
    }
    Ok(serde_json::to_value(jsonld_map).unwrap())
}
