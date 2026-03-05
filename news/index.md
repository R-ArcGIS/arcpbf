# Changelog

## arcpbf 0.2.0

CRAN release: 2025-10-22

- Adds support for 64 bit integers closing
  <https://github.com/R-ArcGIS/arcpbf/issues/15> h/t to
  [@jjoeldaniel](https://github.com/jjoeldaniel) for reporting.

## arcpbf 0.1.7

CRAN release: 2025-04-10

- Handles missing CRS and closes
  <https://github.com/R-ArcGIS/arcpbf/issues/11> h/t
  [@elipousson](https://github.com/elipousson) for reporting
- Returns warning message when `esriFieldBlob` is encountered
  <https://github.com/R-ArcGIS/arcpbf/issues/6>
- Fixes bug where an error occured when a query returned no rows
  <https://github.com/R-ArcGIS/arcpbf/issues/8>

## arcpbf 0.1.6

CRAN release: 2024-10-01

- Adds `tests/` to `.Rbuildignore` to pass CRAN checks

## arcpbf 0.1.5

CRAN release: 2024-09-17

- Addresses CRAN removal for failing to compile on Fedora.

## arcpbf 0.1.4

CRAN release: 2024-08-01

- Addresses MSRV requirement by replacing `std::cell::OnceCell` with
  `once_cell::sync::OnceCell`
- Fix parsing of dates and small integers
- Add minimal integration tests with
  [arcgislayers](https://developers.arcgis.com/r-bridge)

## arcpbf 0.1.3

CRAN release: 2024-07-10

- Closes <https://github.com/R-ArcGIS/arcpbf/issues/2>
- Closes <https://github.com/R-ArcGIS/arcpbf/issues/1>

## arcpbf 0.1.2

CRAN release: 2024-07-05

- Bump version of extendr-api to 0.7.0 to avoid r-devel warnings

## arcpbf 0.1.1

- Fixes a bug where sfg class was not assigned for empty geometries.
- `multi_resp_body_pbf()` becomes
  [`resps_data_pbf()`](../reference/httr2.md) to be more inline with
  `httr2` release
- Fixes a bug when processing a list of protocol buffers that contain
  tables

## arcpbf 0.1.0

CRAN release: 2023-11-09

- Initial CRAN submission.
