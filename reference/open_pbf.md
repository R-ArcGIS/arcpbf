# Read a pbf file as a raw vector

Read a pbf file as a raw vector

## Usage

``` r
open_pbf(path)
```

## Arguments

- path:

  the path to the `.pbf` file.

## Value

a raw vector

## Examples

``` r
count_fp <- system.file("count.pbf", package = "arcpbf")
oid_fp <- system.file("ids.pbf", package = "arcpbf")
tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
fc_fp <- system.file("small-points.pbf", package = "arcpbf")
count_raw <- open_pbf(count_fp)
oid_raw <- open_pbf(oid_fp)
tbl_raw <- open_pbf(tbl_fp)
fc_raw <- open_pbf(fc_fp)
```
