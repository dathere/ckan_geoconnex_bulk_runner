use anyhow::{Result, bail};

// TODO: Ensure error output is only streamed to stderr as per Geoconnex docs

#[tokio::main]
async fn main() -> Result<()> {
    let ckan = ckanaction::CKAN::builder()
        .url("http://localhost:5000")
        .build();

    // Paginate through /api/3/action/package_list until only an empty array is returned
    let mut offset = 0;
    loop {
        // TODO: Verify that only public datasets are returned, otherwise consider /package_search
        let response = ckan.package_list().offset(offset).limit(100).call().await?;
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
                    // 0. Get the dataset name as a string
                    let dataset_name_str = dataset_name.as_str().unwrap();
                    println!("{dataset_name_str}");
                    // TODO: Identify if dataset names are unique
                    // 1. Get the dataset's metadata with /package_show by using the dataset name as the id
                    let dataset_metadata = ckan
                        .package_show()
                        .id(dataset_name_str.to_string())
                        .call()
                        .await?;
                    println!("{dataset_metadata:#?}");
                    // 2. Construct JSON-LD based on the data from /package_show
                    // 3. Validate the JSON-LD against the dataset JSON schema
                    // 4. Print the JSON-LD on a new line to stdout
                }
            }
        } else {
            bail!("CKAN API returned {{\"success\": false\"}}. Full response: {response}");
        }
        offset = offset + 100;
    }

    Ok(())
}
