use anyhow::{Result, bail};
use ckan_geoconnex_bulk_runner::schema::get_location_schema;
use serde_json::json;

#[test]
fn validate_usgs_location_jsonld() -> Result<()> {
    let usgs_location_jsonld = json!({
      "@context": {
        "@vocab": "https://schema.org/",
        "gsp": "http://www.opengis.net/ont/geosparql#",
        "hyf": "https://www.opengis.net/def/schema/hy_features/hyf/",
        "locType": "https://api.waterdata.usgs.gov/ogcapi/v0/collections/site-types/items/"
      },
      "@type": [
        "Place",
        "hyf:HY_HydrometricFeature",
        "hyf:HY_HydroLocation",
        "locType:ST-CA"
      ],
      "@id": "https://geoconnex.us/usgs/monitoring-location/USGS-253937080285200",
      "name": "BLACKCREEKCANALWESTOFSOUTHMIAMI FLA",
      "identifier": {
        "@type": "PropertyValue",
        "propertyID": "USGS site identifier",
        "value": "253937080285200"
      },
      "url": "https://api.waterdata.usgs.gov/ogcapi/v0/collections/monitoring-locations/items/USGS-253937080285200",
      "provider": {
        "@type": "GovernmentOrganization",
        "name": "U.S. Geological Survey"
      },
      "geo": {
        "@type": "GeoCoordinates",
        "latitude": 25.6606597832648,
        "longitude": -80.4808896071386
      },
      "gsp:hasGeometry": {
        "@type": "http://www.opengis.net/ont/sf#Point",
        "gsp:asWKT": {
          "@type": "gsp:wktLiteral",
          "@value": "POINT (-80.4808896071386 25.6606597832648)"
        },
        "gsp:crs": {
          "@id": "http://www.opengis.net/def/crs/OGC/1.3/CRS84"
        }
      }
    });

    let dataset_json_schema = get_location_schema();

    if let Err(e) = jsonschema::validate(&dataset_json_schema, &usgs_location_jsonld) {
        println!("Error during validation:");
        bail!("{e}");
    } else {
        println!("Successfully validated.");
    }

    Ok(())
}
