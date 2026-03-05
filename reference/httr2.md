# Extract PBFs from httr2_response objects

Processes `httr2_response` objects that return FeatureCollection PBFs.

## Usage

``` r
resp_body_pbf(resp, post_process = TRUE, use_sf = TRUE)

resps_data_pbf(resps, post_process = TRUE, use_sf = TRUE)
```

## Arguments

- resp:

  A httr2 [response](https://httr2.r-lib.org/reference/response.html)
  object, created by
  [`req_perform()`](https://httr2.r-lib.org/reference/req_perform.html).

- post_process:

  default `TRUE`. Apply [`post_process_pbf()`](post_process_pbf.md) to
  the pbf body.

- use_sf:

  default `TRUE`. Whether or not to return an `sf` object.

- resps:

  a list of `httr2_response` objects such as created by
  [`httr2::req_perform_parallel()`](https://httr2.r-lib.org/reference/req_perform_parallel.html)

## Value

A processed FeatureCollection pbf. Either a scalar integer, named list,
data.frame, or an sf object if post-processing is applied.

## Details

Responses of type `application/x-protobuf` are automatically processed
using [`process_pbf()`](process_pbf.md) with optional post-processing
applied. Theses functions assume that the body of the responses are an
Esri FeatureCollection protocol buffer.

### Lists of responses

When running multiple requests in parallel using
[`httr2::req_perform_parallel()`](https://httr2.r-lib.org/reference/req_perform_parallel.html)
the responses are returned as a list of responses. `resps_data_pbf()`
processes the responses in a vectorized manner.

Results are post-processed by default and return sf objects if
applicable. This may not be desirable if heterogeneous response types
are expected. For example, if one list element contains a count result
and another contains an object ID result.

See [`post_process_pbf()`](post_process_pbf.md) for more details.

Note: Knowledge Graph protocol buffers and other protobuf formats are
not supported and will result in an error if used with these functions.

## Examples

``` r
if (rlang::is_installed(c("httr2", "sf")) && interactive()) {
  base_url <- file.path(
    "https://services.arcgis.com/P3ePLMYs2RVChkJx",
    "arcgis", "rest", "services",
    "ACS_Population_by_Race_and_Hispanic_Origin_Boundaries",
    "FeatureServer", "2", "query",
    fsep = "/"
  )

  # create the base request
  req <- httr2::request(base_url)

  # fill query parameters
  req <- httr2::req_url_query(
    req,
    where = "1=1",
    outFeilds = "objectid",
    resultRecordCount = 1,
    f = "pbf"
  )

  # make the request
  resp <- httr2::req_perform(req)

  # parse the request
  resp_body_pbf(resp)

  # simulate response from multi_req_perform
  resps <- list(resp, resp, resp)

  # process them all at once
  resps_data_pbf(resps)
}
```
