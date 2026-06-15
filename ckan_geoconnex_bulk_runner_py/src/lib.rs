use pyo3::prelude::*;

/// Python functions for Geoconnex integration that can be used in CKAN extensions.
/// Based on <https://github.com/dathere/ckan_geoconnex_bulk_runner>.
#[pymodule]
mod ckan_geoconnex_bulk_runner_py {
    use pyo3::{exceptions::PyException, prelude::*};

    #[pyfunction]
    /// Construct Geoconnex-compatible JSON-LD as a string from dataset metadata.
    ///
    /// Input: Dataset metadata (output of /package_show for a CKAN dataset) as a string.
    /// Output: Constructed Geoconnex-compatible JSON-LD as a string.
    fn construct_dataset_jsonld_from_metadata(dataset_metadata: String) -> PyResult<String> {
        match serde_json::to_value(dataset_metadata) {
            Ok(dataset_json) => {
                match geoconnex_utils::jsonld::construct_dataset_jsonld_from_metadata(dataset_json)
                {
                    Ok(jsonld) => serde_json::to_string(&jsonld).map_err(|e| {
                        PyException::new_err(format!(
                            "Error when converting JSON-LD to string: {e}"
                        ))
                    }),
                    Err(e) => Err(PyException::new_err(e.to_string())),
                }
            }
            Err(e) => Err(PyException::new_err(e.to_string())),
        }
    }
}
