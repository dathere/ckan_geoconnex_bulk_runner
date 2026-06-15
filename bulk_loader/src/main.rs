use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Get latest release data which is organized as a single JSONL file
    // at https://github.com/dathere/ckan_geoconnex_bulk_runner/releases/latest
    let body = reqwest::get("https://github.com/dathere/ckan_geoconnex_bulk_runner/releases/latest/download/ckan-geoconnex-web-resources.jsonl")
        .await?
        .text()
        .await?;
    // Print each line to stdout
    for line in body.lines() {
        println!("{line}");
    }

    Ok(())
}
