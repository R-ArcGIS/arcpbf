
<!-- badges: start -->

[![R-CMD-check](https://github.com/R-ArcGIS/arcpbf/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/R-ArcGIS/arcpbf/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

# arcpbf

`{arcpbf}` is an R package that processes [Esri FeatureCollection
Protocol
Buffers](https://github.com/Esri/arcgis-pbf/tree/main/proto/FeatureCollection).
It is written in Rust and powered by the
[extendr](https://github.com/extendr/extendr) library.

arcpbf has functions for reading protocol buffer (pbf) results from an
ArcGIS REST API result. pbf results are returned when `f=pbf` in a
[query](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm).

The package is extremely lightweight and fast.

Limitation: this package does not support Z and M dimensions at this
point.

## TL;DR

- `open_pbf()` will read a FeatureCollection `pbf` file into a raw
  vector
- `read_pbf()` will read a FeatureCollection `pbf` file *and* process it
  with
- `resp_body_pbf()` and `resps_data_pbf()` process `httr2_response`
  objects with FeatureCollection pbf bodies
- `process_pbf()` will process a raw vector or a list of raw vectors
- `post_process_pbf()` will apply post processing steps to the results
  of `process_pbf()`
  - set `use_sf = TRUE` to return an `sf` object if possible. Applied by
    default in `read_pbf()`, `resp_body_pbf()` and `resps_data_pbf()`.

> ***Developer Note***: Rust must be installed to compile the package.
> Run the one line installation instructions at <https://rustup.rs/>. To
> verify your Rust installation is compatible, run
> `rextendr::rust_sitrep()`. That’s it.

### PBF support

Note that *only* the FeatureCollection pbf specification is supported by
arcpbf. If you want to process OSM pbf files use
[`osmextract::oe_read()`](https://docs.ropensci.org/osmextract/reference/oe_read.html).
Or, if you want to create and read arbitrary protocol buffers directly
in R, use
[`RprotoBuf`](https://cran.r-project.org/web/packages/RProtoBuf).

## Basic usage

In most cases, we will be processing a protocol buffer directly from an
http request created with [`{httr2}`](https://httr2.r-lib.org/).

``` r
library(arcpbf)

# specify url to sent our request to
url <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/ACS_Population_by_Race_and_Hispanic_Origin_Boundaries/FeatureServer/2/query?where=1=1&outFields=objectid&resultRecordCount=10&f=pbf&token="
req <- httr2::request(url)
resp <- httr2::req_perform(req)

resp
#> <httr2_response>
#> GET
#> https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/ACS_Population_by_Race_and_Hispanic_Origin_Boundaries/FeatureServer/2/query?where=1=1&outFields=objectid&resultRecordCount=10&f=pbf&token=
#> Status: 200 OK
#> Content-Type: application/x-protobuf
#> Body: In memory (60102 bytes)
```

We can process request responses with `resp_body_pbf()`. Post-processing
steps are applied by default. The arguments `post_process` and `use_sf`
are `TRUE` by default.

``` r
resp_body_pbf(resp)
#> Simple feature collection with 10 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17253470 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>    OBJECTID                       geometry
#> 1         1 POLYGON ((-17264972 2244291...
#> 2         2 POLYGON ((-17264972 2244291...
#> 3         3 POLYGON ((-17264587 2241560...
#> 4         4 POLYGON ((-17263053 2239296...
#> 5         5 POLYGON ((-17261894 2236947...
#> 6         6 POLYGON ((-17262143 2241010...
#> 7         7 POLYGON ((-17261575 2233733...
#> 8         8 POLYGON ((-17263397 2234419...
#> 9         9 POLYGON ((-17269918 2239302...
#> 10       10 POLYGON ((-17265323 2239121...
```

### Multiple response objects

When running multiple requests in parallel using
`httr2::req_perform_parallel()` the responses are returned as a list of
responses. `resps_data_pbf()` processes the responses in a vectorized
manner.

``` r
# create a list of requests
reqs <- replicate(5, req, simplify = FALSE)
# perform them in parallel
resps <- httr2::req_perform_parallel(reqs)
#> Iterating ■■■■■■■ 20% | ETA: 7sIterating ■■■■■■■■■■■■■ 40% | ETA: 6sIterating
#> ■■■■■■■■■■■■■■■■■■■■■■■■■ 80% | ETA: 1s

# process the responses 
resps_data_pbf(resps)
#> Simple feature collection with 50 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17253470 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#> First 10 features:
#>    OBJECTID                       geometry
#> 1         1 POLYGON ((-17264972 2244291...
#> 2         2 POLYGON ((-17264972 2244291...
#> 3         3 POLYGON ((-17264587 2241560...
#> 4         4 POLYGON ((-17263053 2239296...
#> 5         5 POLYGON ((-17261894 2236947...
#> 6         6 POLYGON ((-17262143 2241010...
#> 7         7 POLYGON ((-17261575 2233733...
#> 8         8 POLYGON ((-17263397 2234419...
#> 9         9 POLYGON ((-17269918 2239302...
#> 10       10 POLYGON ((-17265323 2239121...
```

### Reading from a file

In some cases you may have a file on disk that you want to process a pbf
from. Use `read_pbf()` to do so. Again, post-processing steps are
applied by default.

``` r
fp <- system.file("small-points.pbf", package = "arcpbf")
read_pbf(fp)
#> Simple feature collection with 2 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>          County                       geometry
#> 1 Hawaii County POLYGON ((-17264972 2244291...
#> 2 Hawaii County POLYGON ((-17264972 2244291...
```

## FeatureCollection Result Types

There are three types of PBF FeatureCollection responses that may be
returned as a result of a [Feature Service Query
request](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm).

- **Feature Results**:
  - the default query response type. Contains individual features with
    their attributes and geometries if available.
- **Count Result**:
  - returned when `returnCountOnly=true` in an API request. Returned as
    a scalar integer vector.
- **Object ID Result**:
  - returned when `returnIdsOnly=true`. A `data.frame` containing object
    IDs where the column name is set to the object ID field name of the
    feature service.

### Feature Results

Feature results can either omit geometry entirely, for example in the
case of a Table or when the query parameter `returnGeometry=false`, or
include it. When geometry is omitted entirely, the response is processed
as a simple `data.frame`. However, if the response does contain
geometry, the response is a bit more complex.

Unprocessed feature results with geometries return a named list with 3
elements:

- `attributes`:
  - a `data.frame` of the fields and their values
- `sr`:
  - a named list with elements `wkt`, `wkid`, `latest_wkid`, `vcs_wkid`,
    and `latest_vcs_wkid`. These determine the coordinate reference
    system of the response as well as the vertical coordinate reference
    system.
- `geometry`:
  - an `sfc` object ***without a computed bounding box or coordinate
    reference system set*** or a CRS set.

``` r
# read an example pbf without post-processing
fc_fp <- system.file("small-points.pbf", package = "arcpbf")
res <- read_pbf(fc_fp, post_process = FALSE)

res
#> $attributes
#>          County
#> 1 Hawaii County
#> 2 Hawaii County
#> 
#> $geometry
#> Geometry set for 2 features 
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: NA ymin: NA xmax: NA ymax: NA
#> CRS:           NA
#> POLYGON ((-17264972 2244291, -17264988 2244297,...
#> POLYGON ((-17264972 2244291, -17264967 2244286,...
#> 
#> $sr
#> $sr$wkt
#> [1] NA
#> 
#> $sr$wkid
#> [1] 102100
#> 
#> $sr$latest_wkid
#> [1] 3857
#> 
#> $sr$vcs_wkid
#> [1] NA
#> 
#> $sr$latest_vcs_wkid
#> [1] NA
```

When post-processing is applied to a geometry Feature Result, the CRS is
set and the bounding box is computed. This requires the `sf` package to
be available.

``` r
post_process_pbf(res)
#> Simple feature collection with 2 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>          County                       geometry
#> 1 Hawaii County POLYGON ((-17264972 2244291...
#> 2 Hawaii County POLYGON ((-17264972 2244291...
```

## Lower level functions

The function `open_pbf()` will read a pbf file into a raw vector which
can be passed to `process_pbf()`. In general you will not need this
function, but it is handy for the sake of example.

``` r
pbf_raw <- open_pbf(fc_fp)
head(pbf_raw, 20)
#>  [1] 12 ac fd 01 0a a8 fd 01 0a 08 4f 42 4a 45 43 54 49 44 12 0c
```

This raw vector can be turned into an R object using `process_pbf()`.
The output *will not* be post processed.

``` r
res <- process_pbf(pbf_raw)
res
#> $attributes
#>          County
#> 1 Hawaii County
#> 2 Hawaii County
#> 
#> $geometry
#> Geometry set for 2 features 
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: NA ymin: NA xmax: NA ymax: NA
#> CRS:           NA
#> POLYGON ((-17264972 2244291, -17264988 2244297,...
#> POLYGON ((-17264972 2244291, -17264967 2244286,...
#> 
#> $sr
#> $sr$wkt
#> [1] NA
#> 
#> $sr$wkid
#> [1] 102100
#> 
#> $sr$latest_wkid
#> [1] 3857
#> 
#> $sr$vcs_wkid
#> [1] NA
#> 
#> $sr$latest_vcs_wkid
#> [1] NA
```

Post-processing can be applied to the result of `process_pbf()` using
`post_process_pbf()`.

``` r
post_process_pbf(res)
#> Simple feature collection with 2 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>          County                       geometry
#> 1 Hawaii County POLYGON ((-17264972 2244291...
#> 2 Hawaii County POLYGON ((-17264972 2244291...
```

`post_process_pbf()` can also be applied to a list of processed pbf
responses.

``` r
multi_res <- list(res, res, res)

post_process_pbf(multi_res)
#> Simple feature collection with 6 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>          County                       geometry
#> 1 Hawaii County POLYGON ((-17264972 2244291...
#> 2 Hawaii County POLYGON ((-17264972 2244291...
#> 3 Hawaii County POLYGON ((-17264972 2244291...
#> 4 Hawaii County POLYGON ((-17264972 2244291...
#> 5 Hawaii County POLYGON ((-17264972 2244291...
#> 6 Hawaii County POLYGON ((-17264972 2244291...
```

## Benchmarking

Below is a bench mark that compares processing pbfs to the current
approach of processing raw json in arcgislayers and arcgisutils. The
below recreates the example from the README of arcgislayers.

``` r
jsn <- function() {
  json_reqs <- c(
    "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_Counties_Generalized_Boundaries/FeatureServer/0/query?outFields=%2A&where=1%3D1&outSR=%7B%22wkid%22%3A4326%7D&returnGeometry=TRUE&token=&f=json&resultOffset=0",
    "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_Counties_Generalized_Boundaries/FeatureServer/0/query?outFields=%2A&where=1%3D1&outSR=%7B%22wkid%22%3A4326%7D&returnGeometry=TRUE&token=&f=json&resultOffset=2001"
  )
  reqs <- lapply(json_reqs, httr2::request) 
  
  resps <- httr2::req_perform_parallel(reqs) |> 
    lapply(function(x) arcgisutils::parse_esri_json(httr2::resp_body_string(x))) 
  
  do.call(rbind.data.frame, resps) |> 
    sf::st_as_sf()
}

# protobuff processing 
pbf <- function() {
  
  pbf_reqs <- c(
    "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_Counties_Generalized_Boundaries/FeatureServer/0/query?outFields=%2A&where=1%3D1&outSR=%7B%22wkid%22%3A4326%7D&returnGeometry=TRUE&token=&f=pbf&resultOffset=0",
    "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_Counties_Generalized_Boundaries/FeatureServer/0/query?outFields=%2A&where=1%3D1&outSR=%7B%22wkid%22%3A4326%7D&returnGeometry=TRUE&token=&f=pbf&resultOffset=2001"
  )
  
  reqs <- lapply(pbf_reqs, httr2::request)
  
  httr2::req_perform_parallel(reqs) |> 
    resps_data_pbf()
}

bench::mark(
  jsn(),
  pbf(),
  check = FALSE,
  relative = TRUE,
  iterations = 5
)
#> Iterating ■■■■■■■■■■■■■■■■                  50% | ETA:  1s                                                           Iterating ■■■■■■■■■■■■■■■■                  50% | ETA:  1s
#> Iterating ■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■  100% | ETA:  0s
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
#> # A tibble: 2 × 6
#>   expression   min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>    <dbl>
#> 1 jsn()       3.06   3.27      1         3.94     1.39
#> 2 pbf()       1      1         3.94      1        1
```

## Internals

Internally, there is a rust crate [`esripbf`](./src/rust/esripbf) which
is a a Rust library built with
[`prost`](https://github.com/tokio-rs/prost) to handle the
[FeatureCollection Protocol Buffer
Specification](https://github.com/Esri/arcgis-pbf/tree/main/proto/FeatureCollection).

## Future Notes

Alternatively, it may make sense to write to a geoarrow array and
convert to sfc using {wk}. These are just thoughts.
