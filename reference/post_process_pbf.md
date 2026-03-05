# Post process pbf results

Applies post-processing to the results of
[`process_pbf()`](process_pbf.md)

## Usage

``` r
post_process_pbf(x, use_sf = TRUE)
```

## Arguments

- x:

  an object as returned by [`process_pbf()`](process_pbf.md) or
  [`read_pbf()`](read_pbf.md)

- use_sf:

  default `TRUE`. Whether or not to return an `sf` object.

## Value

An object of class `data.frame`, `sf`, or a scalar integer vector.

See [`process_pbf()`](process_pbf.md) for more details.

## Details

If `x` is a list object, the results will be row-binded. This is
appropriate if each element in the list is a `data.frame` or a feature
result with geometry. However, if each element is *not* the same, the
post-processing *will* error. If you cannot be certain that all elements
that you will be post processing will be the same, post-process each
list element independently.

## Examples

``` r
tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
fc_fp <- system.file("small-points.pbf", package = "arcpbf")

# table feature collection
fc <- read_pbf(tbl_fp)
head(post_process_pbf(fc))
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
fc <- read_pbf(fc_fp)
head(post_process_pbf(fc))
#> Simple feature collection with 2 features and 1 field
#> Geometry type: POLYGON
#> Dimension:     XY
#> Bounding box:  xmin: -17298700 ymin: 2216212 xmax: -17260020 ymax: 2261306
#> Projected CRS: WGS 84 / Pseudo-Mercator
#>          County                       geometry
#> 1 Hawaii County POLYGON ((-17264972 2244291...
#> 2 Hawaii County POLYGON ((-17264972 2244291...
```
