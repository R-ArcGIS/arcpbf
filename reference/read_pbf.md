# Read a FeatureCollection Protocol Buffer

Given a binary file containing a FeatureCollection protocol buffer
(pbf), read its contents into R as an R object.

## Usage

``` r
read_pbf(path, post_process = TRUE, use_sf = TRUE)
```

## Arguments

- path:

  a scalar character of the path to the pbf file

- post_process:

  default `TRUE`. Apply [`post_process_pbf()`](post_process_pbf.md) to
  the pbf body.

- use_sf:

  default `TRUE`. Whether or not to return an `sf` object.

## Value

Either a data.frame, list, scalar integer, or sf object if
`post_process = TRUE` and `use_sf = TRUE`.

See [`process_pbf()`](process_pbf.md) for more.

## Examples

``` r
count_fp <- system.file("count.pbf", package = "arcpbf")
oid_fp <- system.file("ids.pbf", package = "arcpbf")
tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
fc_fp <- system.file("small-points.pbf", package = "arcpbf")

# count response
read_pbf(count_fp)
#> [1] 3143

# object id response
head(read_pbf(oid_fp))
#>   OBJECTID
#> 1        1
#> 2        2
#> 3        3

# table feature collection
read_pbf(tbl_fp)
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

# feature collection with geometry
read_pbf(fc_fp)
#> Simple feature collection with 2 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>          County                       geometry
#> 1 Hawaii County POLYGON ((-17264972 2244291...
#> 2 Hawaii County POLYGON ((-17264972 2244291...
```
