# arcpbf 0.1.6

* Adds `tests/` to `.Rbuildignore` to pass CRAN checks

# arcpbf 0.1.5

* Addresses CRAN removal for failing to compile on Fedora.

# arcpbf 0.1.4

* Addresses MSRV requirement by replacing `std::cell::OnceCell` with `once_cell::sync::OnceCell`
* Fix parsing of dates and small integers 
* Add minimal integration tests with `{arcgislayers}`

# arcpbf 0.1.3

* Closes https://github.com/R-ArcGIS/arcpbf/issues/2
* Closes https://github.com/R-ArcGIS/arcpbf/issues/1

# arcpbf 0.1.2

* Bump version of extendr-api to 0.7.0 to avoid r-devel warnings

# arcpbf 0.1.1

* Fixes a bug where sfg class was not assigned for empty geometries. 
* `multi_resp_body_pbf()` becomes `resps_data_pbf()` to be more inline with `httr2` release
* Fixes a bug when processing a list of protocol buffers that contain tables

# arcpbf 0.1.0

* Initial CRAN submission.
