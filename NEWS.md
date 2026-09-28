# arcpbf (development version)

* Updates to the current Esri `FeatureCollection.proto`, which adds the `esriFieldTypeBigInteger`, `esriFieldTypeDateOnly`, `esriFieldTypeTimeOnly`, and `esriFieldTypeTimestampOffset` field types. Previously these were all read as small integers.
  * `esriFieldTypeDateOnly` fields are now returned as `Date` instead of `POSIXct`.
  * `esriFieldTypeTimestampOffset` fields are now returned as `POSIXct` in UTC instead of `NA`. The original offset is not kept.
  * `esriFieldTypeTimeOnly` fields are now returned as character (e.g. `"14:35:00.927"`) instead of `NA`.
  * `esriFieldTypeBigInteger` fields are now returned as doubles.
* `esriFieldTypeXML` fields are now returned as character instead of an empty list with an unsupported field type message.
* `esriFieldTypeSmallInteger` and `esriFieldTypeInteger` fields are no longer classed as `POSIXct` when every value is a date string. Non-numeric strings in these fields are `NA`. This removes the workaround for <https://github.com/R-ArcGIS/arcgislayers/issues/211>, which was caused by date-only fields being misread as small integers.
* Explicit null values sent by newer services are now read as `NA`.

# arcpbf 0.3.0

* Improves error handling when encountering parse errors.
* Adds support for Windows ARM <https://github.com/R-ArcGIS/arcpbf/pull/20> @jeroen
* Integer fields are no longer classed as dates when a value arrives as a string. The whole column was stamped `POSIXct` while its values stayed raw, so `34` rendered as `1969-12-31 17:00:34` <https://github.com/R-ArcGIS/arcgislayers/issues/211>

# arcpbf 0.2.0

* Fixes bug where datasets with CRSs within the ESRI authority would be returned with missing CRSs when converted to sf objects.
* Adds support for 64 bit integers closing <https://github.com/R-ArcGIS/arcpbf/issues/15> h/t to @jjoeldaniel for reporting.

# arcpbf 0.1.7

* Handles missing CRS and closes <https://github.com/R-ArcGIS/arcpbf/issues/11> h/t @elipousson for reporting
* Returns warning message when `esriFieldBlob` is encountered <https://github.com/R-ArcGIS/arcpbf/issues/6>
* Fixes bug where an error occured when a query returned no rows <https://github.com/R-ArcGIS/arcpbf/issues/8>

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
