
<!-- badges: start -->

[![R-CMD-check](https://github.com/JosiahParry/arcpbf/actions/workflows/R-CMD-check.yaml/badge.svg)](https://github.com/JosiahParry/arcpbf/actions/workflows/R-CMD-check.yaml)
<!-- badges: end -->

# arcpbf

`{arcpbf}` is an R package that processes Esri Protocol Buffers. It is
written in Rust and powered by the
[extendr](https://github.com/extendr/extendr) library.

arcpbf has functions for reading protocol buffer (pbf) results from an
ArcGIS REST API result. pbf results are returned when `f=pbf` in a
[query](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm).
The package is intended to be extremely lightweight and fast. As such,
it has no hard dependencies.

> ***Important***: Rust must be installed to compile the package. Run
> the one line installation instructions at <https://rustup.rs/>. To
> verify your Rust installation is compatible, run
> `rextendr::rust_sitrep()`. That’s it.

## TL;DR

- `open_pbf()` will read a FeatureCollection `pbf` file into a raw
  vector
- `read_pbf()` will read a FeatureCollection `pbf` file *and* process it
- `process_pbf()` will process a raw vector or a list of raw vectors
- `post_process_pbf()` will apply post processing steps to the results
  of `process_pbf()`
  - set `use_sf = FALSE` to return a `data.frame` otherwise an `sf`
    object will be returned
- `resp_body_pbf()` process an `httr2_response` as a pbf
- `multi_resp_process()` processes a list of `httr2_response` using
  `process_pbf()`
  - if an element is not a response or have a 200 status code, `NULL` is
    returned

## Basic usage

There are two ways of processing a FeatureCollection pbf. We either can
read directly from a binary file (typically with a `.pbf` extension).
Or, we process the a raw vector containing the binary of a pbf as
returned by an http request. These are accomplished with `read_pbf()`
and `process_pbf()` respectively.

Here we read a single pbf file.

``` r
library(arcpbf)

x <- read_pbf("inst/pbfs/pnts.pbf")
str(x, 1)
#> List of 3
#>  $ attributes:'data.frame':  10 obs. of  18 variables:
#>  $ geometry  :List of 10
#>   ..- attr(*, "class")= chr [1:2] "sfc_POINT" "sfc"
#>   ..- attr(*, "precision")= num 0
#>   ..- attr(*, "n_empty")= int 0
#>   ..- attr(*, "bbox")= Named num [1:4] NA NA NA NA
#>   .. ..- attr(*, "names")= chr [1:4] "xmin" "ymin" "xmax" "ymax"
#>   ..- attr(*, "crs")=List of 2
#>   .. ..- attr(*, "class")= chr "crs"
#>  $ sr        :List of 5
```

For pbf files containing geometries, we retrieve a list of 3 elements:

- `attributes` is a `data.frame` of the fields of the FeatureCollection
- `geometry` is a pseudo-sfc object (more on this later) which is a list
  of sfg geometry objects
- `sr` is a named list of the spatial reference of the feature
  collection

Whereas FeatureCollections *without geometries* will always return a
single data.frame.

``` r
read_pbf("inst/pbfs/small-table.pbf")
#>   OBJECTID       Adoption_Service_Provider  DBA       city                state
#> 1        1 A Family in Bloom Adoption, LLC <NA>    Boulder             Colorado
#> 2        2     A Love Beyond Borders, Inc. <NA>     Denver             Colorado
#> 3        3                  Adopolis, Inc. <NA> Washington District of Columbia
#>    Accreditation_or_Approval_Statu Accredited_Approvedto_Provide
#> 1 Accredited/Approved – In Process         Incoming and Outgoing
#> 2                         Approved                      Incoming
#> 3                       Accredited                      Incoming
#>                                           full_address
#> 1           Adoption Service Provider,Boulder,Colorado
#> 2      A Family in Bloom Adoption, LLC,Denver,Colorado
#> 3 A Love Beyond Borders, Inc.,Washington,Washington DC
```

These results use no external dependencies. If we want to convert the
named list into a familiar sf object we will have to **post-process** it
using `post_process_pbf()`. The `use_sf` argument, which defaults to
`TRUE`, will use the `sf` package to return an `sf` object.

``` r
post_process_pbf(x)
#> Simple feature collection with 10 features and 18 fields
#> Geometry type: POINT
#> Dimension:     XY
#> Bounding box:  xmin: NA ymin: NA xmax: NA ymax: NA
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>           id          retailer                fascia
#> 1 1010001841 Marks and Spencer Marks and Spencer MSA
#> 2 1010000921              Asda      Asda Supercentre
#> 3 1010003178        Sainsburys            Sainsburys
#> 4 1010014115           Budgens               Budgens
#> 5 1010015258              Lidl                  Lidl
#>                      store_name                  add_one          add_two
#> 1   M&S Reading East M4 Moto SF Reading East Services M4             <NA>
#> 2 Asda Lower Earley Supercentre             Chalfort Way     Lower Earley
#> 3             Sainsburys Calcot                Bath Road           Calcot
#> 4      Budgens Three Mile Cross         Basingstoke Road Three Mile Cross
#> 5           Lidl Calcot Reading                Bath Road             <NA>
#>               town suburb postcode   long_wgs  lat_wgs    bng_e    bng_n
#> 1          Reading Calcot RG30 3UQ -1.0350837 51.42616 467183.5 170124.7
#> 2          Reading Earley  RG6 5TT -0.9327606 51.42482 474299.7 170075.0
#> 3          Reading Calcot RG31 7SA -1.0614843 51.44304 465324.0 171978.0
#> 4 Three Mile Cross   <NA>  RG7 1BA -0.9739431 51.40521 471466.9 167853.1
#> 5          Reading Calcot RG30 2HB -1.0244860 51.44386 467894.1 172102.9
#>                            pqi open_date                              size_band
#> 1 Rooftop geocoded by Geolytix      <NA>                    < 3,013 ft2 (280m2)
#> 2 Rooftop geocoded by Geolytix      <NA>                30,138 ft2 > (2,800 m2)
#> 3 Rooftop geocoded by Geolytix      <NA>                30,138 ft2 > (2,800 m2)
#> 4 Rooftop geocoded by Geolytix  20169999    3,013 < 15,069 ft2 (280 < 1,400 m2)
#> 5 Rooftop geocoded by Geolytix  20171026 15,069 < 30,138 ft2 (1,400 < 2,800 m2)
#>      county ObjectId                  geometry
#> 1 Berkshire        1   POINT (-115225 6697025)
#> 2 Berkshire        2 POINT (-103834.4 6696787)
#> 3 Berkshire        3 POINT (-118163.9 6700039)
#> 4 Berkshire        4 POINT (-108418.9 6693287)
#> 5 Berkshire        5 POINT (-114045.3 6700186)
#>  [ reached 'max' / getOption("max.print") -- omitted 5 rows ]
```

If it is set to `FALSE`, it will return a `data.frame`.

``` r
post_process_pbf(x, use_sf = FALSE)
#>           id          retailer                fascia
#> 1 1010001841 Marks and Spencer Marks and Spencer MSA
#> 2 1010000921              Asda      Asda Supercentre
#> 3 1010003178        Sainsburys            Sainsburys
#> 4 1010014115           Budgens               Budgens
#> 5 1010015258              Lidl                  Lidl
#>                      store_name                  add_one          add_two
#> 1   M&S Reading East M4 Moto SF Reading East Services M4             <NA>
#> 2 Asda Lower Earley Supercentre             Chalfort Way     Lower Earley
#> 3             Sainsburys Calcot                Bath Road           Calcot
#> 4      Budgens Three Mile Cross         Basingstoke Road Three Mile Cross
#> 5           Lidl Calcot Reading                Bath Road             <NA>
#>               town suburb postcode   long_wgs  lat_wgs    bng_e    bng_n
#> 1          Reading Calcot RG30 3UQ -1.0350837 51.42616 467183.5 170124.7
#> 2          Reading Earley  RG6 5TT -0.9327606 51.42482 474299.7 170075.0
#> 3          Reading Calcot RG31 7SA -1.0614843 51.44304 465324.0 171978.0
#> 4 Three Mile Cross   <NA>  RG7 1BA -0.9739431 51.40521 471466.9 167853.1
#> 5          Reading Calcot RG30 2HB -1.0244860 51.44386 467894.1 172102.9
#>                            pqi open_date                              size_band
#> 1 Rooftop geocoded by Geolytix      <NA>                    < 3,013 ft2 (280m2)
#> 2 Rooftop geocoded by Geolytix      <NA>                30,138 ft2 > (2,800 m2)
#> 3 Rooftop geocoded by Geolytix      <NA>                30,138 ft2 > (2,800 m2)
#> 4 Rooftop geocoded by Geolytix  20169999    3,013 < 15,069 ft2 (280 < 1,400 m2)
#> 5 Rooftop geocoded by Geolytix  20171026 15,069 < 30,138 ft2 (1,400 < 2,800 m2)
#>      county ObjectId                  geometry
#> 1 Berkshire        1   POINT (-115225 6697025)
#> 2 Berkshire        2 POINT (-103834.4 6696787)
#> 3 Berkshire        3 POINT (-118163.9 6700039)
#> 4 Berkshire        4 POINT (-108418.9 6693287)
#> 5 Berkshire        5 POINT (-114045.3 6700186)
#>  [ reached 'max' / getOption("max.print") -- omitted 5 rows ]
```

## Types of FeatureCollection results

The FeatureCollection pbf can return three different types of results.
They can be query results as above, or they can also be **count** or
**Object ID** results.

Counts will always return a scalar integer vector.

``` r
read_pbf("inst/pbfs/count.pbf")
#> [1] 3143
```

Whereas the Object ID result type will return a `data.frame` with a
single column. The column name is the name of the object ID field and
the values are the object IDs as a numeric vector.

``` r
ids <- read_pbf("inst/pbfs/ids.pbf")
head(ids)
#>   OBJECTID
#> 1       93
#> 2       73
#> 3       83
#> 4       86
#> 5     2671
#> 6     3140
```

## Reading from a raw vector

The `open_pbf()` function will read a pbf file into a raw vector which
can be passed to `process_pbf()`. In general you will not need this
function, but it is handy for the sake of example.

``` r
pbf_raw <- open_pbf("inst/pbfs/big-pnts.pbf")
head(pbf_raw, 20)
#>  [1] 12 aa ec 10 0a a6 ec 10 0a 08 4f 62 6a 65 63 74 49 64 12 0c
```

This raw vector can be turned into an R object using `process_pbf()`.

``` r
res <- process_pbf(pbf_raw)
str(res, 1)
#> List of 3
#>  $ attributes:'data.frame':  1000 obs. of  18 variables:
#>  $ geometry  :sfc_POINT of length 1000; first list element:  'XY' num [1:2] -115225 6697025
#>  $ sr        :List of 5
```

## Processing requests

The true purpose of this package is to process requests from the REST
API. Here we process a single request using
[`{httr2}`](https://httr2.r-lib.org/)

``` r
url <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/ACS_Population_by_Race_and_Hispanic_Origin_Boundaries/FeatureServer/2/query?where=1=1&outFields=objectid&f=pbf&token="

resp <- httr2::request(url) |> 
  httr2::req_perform() |> 
  httr2::resp_body_raw()

x <- process_pbf(resp)
```

In developing an R package, one may be creating multiple requests in
parallel using `httr2::multi_req_perform()` as is done in
[`{arcgislayers}`](https://github.com/R-ArcGIS/arcgislayers).

``` r
reqs <- replicate(5, httr2::request(url), simplify = FALSE)

resps <- httr2::multi_req_perform(reqs)
```

We can process all of the responses using `multi_resp_process()` and
pass the results to `post_process_pbf()`.

Note that when post processing a list of responses,
`data.table::rbindlist()` will be used to bind the results together. If
data.table is not available, `dplyr::bind_rows()` will be used. If dplyr
is not available, rows will be bound together using
`do.call(rbind.data.frame, x)`.

``` r
res <- multi_resp_process(resps) |> 
  post_process_pbf()

head(res)
#> Simple feature collection with 6 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17253470 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>   OBJECTID                       geometry
#> 1        1 POLYGON ((-17264972 2244291...
#> 2        2 POLYGON ((-17264972 2244291...
#> 3        3 POLYGON ((-17264587 2241560...
#> 4        4 POLYGON ((-17263053 2239296...
#> 5        5 POLYGON ((-17261894 2236947...
#> 6        6 POLYGON ((-17262143 2241010...
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
  
  resps <- httr2::multi_req_perform(reqs) |> 
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
  
  httr2::multi_req_perform(reqs) |> 
    multi_resp_process() |> 
    post_process_pbf()
}

bench::mark(
  jsn(),
  pbf(),
  check = FALSE,
  relative = TRUE,
  iterations = 5
)
#> Warning: Some expressions had a GC in every iteration; so filtering is
#> disabled.
#> # A tibble: 2 × 6
#>   expression   min median `itr/sec` mem_alloc `gc/sec`
#>   <bch:expr> <dbl>  <dbl>     <dbl>     <dbl>    <dbl>
#> 1 jsn()       4.25   3.88      1         4.22      Inf
#> 2 pbf()       1      1         3.84      1         NaN
```

## Internals

Internally, there is a rust crate [`esripbf`](./src/rust/esripbf) which
is a a Rust library built with
[`prost`](https://github.com/tokio-rs/prost) to handle the
[FeatureCollection Protocol Buffer
Specification](https://github.com/Esri/arcgis-pbf/tree/main/proto/FeatureCollection).

## Notes

Alternatively, it may make sense to write to a geoarrow array and
convert to sfc using {wk}. These are just thoughts.
