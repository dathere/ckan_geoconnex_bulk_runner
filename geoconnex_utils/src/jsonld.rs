use anyhow::{Result, bail};
use duct::cmd;
use serde_json::{Value};

pub fn validate_jsonld_with_nabu(jsonld: &Value) -> Result<()> {
    let validation_output = cmd("docker", &[
        "run",
        "internetofwater/nabu:latest",
        "shacl",
        serde_json::to_string(jsonld)?.as_str()
    ]).unchecked().run()?;
    if validation_output.status.success() {
        Ok(())
    } else {
        bail!(format!("Error while validating JSON-LD with nabu: {}", String::from_utf8(validation_output.stderr)?))
    }
}
