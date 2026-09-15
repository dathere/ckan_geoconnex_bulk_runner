# ckan_geoconnex_bulk_runner

https://github.com/user-attachments/assets/779fe866-d511-44f3-91a9-a1c2e1cfa189

> Status: This codebase is currently a work in progress and more documentation is planned.

The `ckan_geoconnex_bulk_runner` codebase is part of a multi-software-system infrastructure to sync water data hubs using [CKAN](https://ckan.org) to the [Geoconnex](https://geoconnex.us) knowledge graph using the [ckanext-gztr](https://gztr.dathere.com) CKAN extension.

- [**generate_release**](generate_release): Compatible CKAN datasets for all connected water data hubs are uploaded to a JSONL file in the latest GitHub release.
- [**bulk_loader**](bulk_loader): Requests and outputs the latest JSONL file from the latest GitHub release. This is ran as a Docker container for each CKAN instance by Geoconnex on a periodic frequency to upload all water data hub web resources to the Geoconnex knowledge graph following the Geoconnex [bulk contribution specification](https://docs.geoconnex.us/contributing/bulk/).
- [**geoconnex_utils**](geoconnex_utils): Helper functions used throughout the ckan_geoconnex_bulk_runner project including JSON-LD construction and JSON schema validation.

This runner is implemented for a water data hub with the relevant fields and/or [ckanext-gztr](https://gztr.dathere.com). For questions reach out to [datHere](https://dathere.com), [Center for Geospatial Solutions](https://cgsearth.org/), or add an issue/discussion.

## Acknowledgements

The ckan_geoconnex_bulk_runner software system is built in collaboration with several organizations. Please view the bottom of [gztr.dathere.com](https://gztr.dathere.com) to learn more about each organization.
