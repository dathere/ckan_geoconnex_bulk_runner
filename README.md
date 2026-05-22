# ckan_geoconnex_bulk_runner

https://github.com/user-attachments/assets/779fe866-d511-44f3-91a9-a1c2e1cfa189

> Status: This codebase is currently a work in progress and more documentation is planned.

The `ckan_geoconnex_bulk_runner` codebase is meant to run as a container for a bulk integration of a [CKAN](https://ckan.org) instance's relevant datasets and vector geospatial features (e.g. for water data hubs) to the [Geoconnex](https://geoconnex.us) knowledge graph. The codebase ultimately runs as a program outputting to standard output JSON-LD on a new line for each approved dataset/location which the Geoconnex crawler then uses to update the Geoconnex knowledge graph.

Refer to the "Contributing via Bulk Containers" documentation here for more information: https://docs.geoconnex.us/contributing/bulk/

This runner is expected to be implemented for a water data hub with the relevant fields and/or ckanext-gztr (not open-source yet) and/or [DataPusher+](https://github.com/dathere/datapusher-plus) enabled. For questions reach out to [datHere](https://dathere.com), [Center for Geospatial Solutions](https://cgsearth.org/), or add an issue/discussion.

## Installation and setup

```bash
cargo run -p ckan_geoconnex_bulk_runner --release
```

To ignore standard error output and only show valid output:

```bash
cargo run -p ckan_geoconnex_bulk_runner --release  2>/dev/null
```

## Run tests

```bash
cargo test -p ckan_geoconnex_bulk_runner
```

To include print statements in test output, run:

```bash
cargo test -p ckan_geoconnex_bulk_runner -- --nocapture
```

If you have the local dump files setup available you can run those tests with:

```bash
cargo test -p ckan_geoconnex_bulk_runner -F local -- --nocapture
```
