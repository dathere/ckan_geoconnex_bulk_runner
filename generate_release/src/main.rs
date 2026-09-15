use anyhow::{Result, bail};
use geoconnex_utils::{jsonld::{validate_jsonld_with_nabu}};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let namespace = env!("NAMESPACE");
    let token = env!("API_TOKEN");
    let instance_url = env!("INSTANCE_URL");
    let mut headers = HashMap::new();
    headers.insert("x-geoconnex-runner".to_string(), token.to_string());

    let ckan = ckanaction::CKAN::builder()
        .url(instance_url)
        .headers(headers)
        .build();

    // Paginate through /api/3/action/package_list until only an empty array is returned
    let mut offset = 0;
    let limit = 100;
    loop {
        // TODO: Verify that only public datasets are returned
        let response = ckan
            .package_list()
            .offset(offset)
            .limit(limit)
            .call()
            .await?;
        // Verify successful response from CKAN API
        let Some(success_opt) = response.get("success") else {
            bail!("CKAN API did not return `success` key. Full response: {response}");
        };
        let Some(success) = success_opt.as_bool() else {
            bail!(
                "Could not parse success key as boolean from CKAN API. Full response: {response}"
            );
        };
        if success {
            let Some(result) = response.get("result") else {
                bail!("CKAN API did not return `result` key. Full response: {response}");
            };
            // Retrieve dataset names from current pagination
            let dataset_names = result.as_array().unwrap();
            if dataset_names.is_empty() {
                break;
            } else {
                // For each dataset in current pagination:
                for dataset_name in dataset_names {
                    // 1. Construct JSON-LD based on the data from /gztr_geoconnex_dataset_jsonld
                    let geoconnex_jsonld_response = ckan.gztr_geoconnex_dataset_jsonld()
                        .id(dataset_name.as_str().unwrap().to_string())
                        .call()
                        .await?;
                    let Some(success) = geoconnex_jsonld_response.get("success") else {
                        bail!(
                            "CKAN API did not return success key in /gztr_geoconnex_dataset_jsonld response for dataset {dataset_name}. Full response: {response}"
                        );
                    };
                    if success.as_bool().unwrap() {
                        let Some(jsonld) = geoconnex_jsonld_response.get("result") else {
                            bail!(
                                "CKAN API did not return result object in /gztr_geoconnex_dataset_jsonld response for dataset {dataset_name}. Full response: {response}"
                            );
                        };
                        // 2. Validate the JSON-LD against the nabu SHACL validation Go CLI tool
                        if validate_jsonld_with_nabu(&jsonld).is_ok() {
                            // 3. Print the JSON-LD on a new line to stdout
                            println!("{jsonld}");
                        } else {
                            eprintln!("JSON-LD for {dataset_name} is not valid.");
                            // eprintln!("{jsonld}");
                        }
                    } else {
                        bail!(
                            "CKAN API returned {{\"success\": false\"}} for /gztr_geoconnex_dataset_jsonld endpoint on dataset {dataset_name}. Full response: {response}"
                        );
                    }
                }
            }
        } else {
            bail!(
                "CKAN API returned {{\"success\": false\"}} for /package_list endpoint. Full response: {response}"
            );
        }
        offset = offset + limit;
    }

    Ok(())
}
