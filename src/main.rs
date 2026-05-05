use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ckan = ckanaction::CKAN::builder()
        .url("http://localhost:5000")
        .build();

    // Paginate through /api/3/action/package_list until only an empty array is returned
    let response = ckan.package_list().call().await?;

    let result = response
        .as_object()
        .unwrap()
        .get("result")
        .unwrap()
        .as_array()
        .unwrap();

    println!("{result:#?}");

    Ok(())
}
