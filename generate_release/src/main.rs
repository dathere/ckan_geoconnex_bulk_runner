use anyhow::{Result, bail};
use geoconnex_utils::{jsonld::construct_dataset_jsonld_from_metadata, schema::get_dataset_schema};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // Identify required header data
    let Ok(nmwdc_token) = std::env::var("NMWDC_API_BULK_LOADER_TOKEN") else {
        bail!("Could not find environment variable NMWDC_API_BULK_LOADER_TOKEN.");
    };
    let mut headers = HashMap::new();
    headers.insert("x-geoconnex-runner".to_string(), nmwdc_token);

    let ckan = ckanaction::CKAN::builder()
        .url("https://catalog.newmexicowaterdata.org")
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
                    // 1. Get the dataset's metadata with /package_show by using the dataset name as the id
                    // TODO: Identify if dataset names are unique
                    let package_show_response = ckan
                        .package_show()
                        .id(dataset_name.as_str().unwrap().to_string())
                        .call()
                        .await?;
                    let Some(success) = package_show_response.get("success") else {
                        bail!(
                            "CKAN API did not return success key in /package_show response for dataset {dataset_name}. Full response: {response}"
                        );
                    };
                    if success.as_bool().unwrap() {
                        let Some(dataset_metadata) = package_show_response.get("result") else {
                            bail!(
                                "CKAN API did not return result object in /package_show response for dataset {dataset_name}. Full response: {response}"
                            );
                        };
                        // 2. Construct JSON-LD based on the data from /package_show
                        let jsonld = match construct_dataset_jsonld_from_metadata(
                            dataset_metadata.to_owned(),
                        ) {
                            Ok(j) => j,
                            Err(e) => {
                                eprintln!(
                                    "Error while attempting to construct JSON-LD from dataset's metadata: {e}"
                                );
                                continue;
                            }
                        };
                        // 3. Validate the JSON-LD against the dataset JSON schema
                        if jsonschema::validate(&get_dataset_schema(), &jsonld).is_ok() {
                            // 4. Print the JSON-LD on a new line to stdout
                            println!("{jsonld}");
                        } else {
                            eprintln!("JSON-LD for {dataset_name} is not valid.");
                            // eprintln!("{jsonld}");
                        }
                    } else {
                        bail!(
                            "CKAN API returned {{\"success\": false\"}} for /package_show endpoint on dataset {dataset_name}. Full response: {response}"
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
