# ckan_geoconnex_bulk_runner

> Status: This codebase is currently a work in progress and more documentation is planned.

The `ckan_geoconnex_bulk_runner` codebase is meant to run as a container for a bulk integration of a [CKAN](https://ckan.org) instance's relevant datasets and locations (e.g. for water data hubs) to the [Geoconnex](https://geoconnex.us) knowledge graph. The codebase ultimately runs as a program outputting to standard output JSON-LD on a new line for each approved dataset/location which the Geoconnex crawler then uses to update the Geoconnex knowledge graph.

Refer to the "Contributing via Bulk Containers" documentation here for more information: https://docs.geoconnex.us/contributing/bulk/

This runner is expected to be implemented for a water data hub with [DataPusher+](https://github.com/dathere/datapusher-plus) enabled. For questions reach out to [datHere](https://dathere.com), [Center for Geospatial Solutions](https://cgsearth.org/), or add an issue/discussion.

## Installation and setup

```bash
cargo run --release
```
