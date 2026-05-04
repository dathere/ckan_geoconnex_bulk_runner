mod utils;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ckan = ckanaction::CKAN::builder()
        .url("https://data.dathere.com")
        .build();
    // TODO: Paginate through package_list and run package_show for each package
    // If about exists then construct JSON-LD and validate then output JSON-LD to stdout on a new line
    let response = ckan.package_show().id("".to_string()).call().await?;

    let result = response
        .as_object()
        .unwrap()
        .get("result")
        .unwrap()
        .as_object()
        .unwrap();
    if let Some(geoconnex_about) = result.get("geoconnex_about") {
        // Check if at least one valid reference feature exists in dataset metadata
    }
    // TODO: Construct JSON-LD if valid `about`
    // TODO: Validate constructed JSON-LD against JSON schema
    // TODO: Print JSON-LD to new line

    println!("{result:#?}");

    Ok(())
}
