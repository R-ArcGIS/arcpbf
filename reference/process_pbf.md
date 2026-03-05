# Process a FeatureCollection PBF

Process a pbf from a raw vector or a list of raw vectors.

## Usage

``` r
process_pbf(proto)
```

## Arguments

- proto:

  either a raw vector or a list of raw vectors containing a
  FeatureCollection pbf

## Value

- For count results, a scalar integer.

- For object ID results a `data.frame` with one column.

- For pbfs that contain geometries, a list of 3 elements:

  - `attributes` is a `data.frame` of the fields of the
    FeatureCollection

  - `geometry` is an sfc object ***without a computed bounding box or
    coordinate reference system set***

  - `sr` is a named list of the spatial reference of the feature
    collection

**Important**: Use [`post_process_pbf()`](post_process_pbf.md) to
convert to an `sf` object with a computed bounding box and CRS.

## Details

There are three types of PBF FeatureCollection responses that may be
returned.

### Feature Result

In the case the PBF is a `FeatureResult` and `use_sf = FALSE`, a
`data.frame` is returned with the spatial reference stored in the `crs`
attribute. Otherwise an `sf` object is returned.

### Count Result

The PBF can also return a count result, for example if the [query
parameter](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm)
`returnCountOnly` is set to `true`. In this case, a scalar integer
vector is returned.

### Object ID Result

In the case that the query parameter `returnIdsOnly` is `true`, a
`data.frame` is returned containing the object IDs and the column name
set to the object ID field name in the feature service.

## Examples

``` r
count_fp <- system.file("count.pbf", package = "arcpbf")
oid_fp <- system.file("ids.pbf", package = "arcpbf")
tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
fc_fp <- system.file("small-points.pbf", package = "arcpbf")

# count response
count_raw <- open_pbf(count_fp)
process_pbf(count_raw)
#> [1] 3143

# object id response
oid_raw <- open_pbf(oid_fp)
head(process_pbf(oid_raw))
#>   OBJECTID
#> 1        1
#> 2        2
#> 3        3

# table feature collection
tbl_raw <- open_pbf(tbl_fp)
process_pbf(tbl_raw)
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
fc_raw <- open_pbf(fc_fp)
process_pbf(fc_raw)
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
#> 
#> 
```
