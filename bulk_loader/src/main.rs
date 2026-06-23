use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Get the CKAN instance's Geoconnex namespace to filter for its JSON-LD data
    let namespace = std::env!("NAMESPACE");
    // Get latest release data which is organized as a single JSONL file
    // at https://github.com/dathere/ckan_geoconnex_bulk_runner/releases/latest
    let body = reqwest::get(format!("https://github.com/dathere/ckan_geoconnex_bulk_runner/releases/latest/download/{namespace}.jsonl"))
        .await?
        .text()
        .await?;
    // Print each line to stdout
    for line in body.lines() {
        println!("{line}");
    }

    Ok(())
}
