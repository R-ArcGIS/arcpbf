
<!-- README.md is generated from README.Rmd. Please edit that file -->

# arcpbf

`{arcpbf}` is an R package that processes Esri Protocol Buffers.

Internally, there is a rust crate [`esripbf`](./src/rust/esripbf) which
is a a Rust library built with `prost` to handle the [FeatureCollection
Protocol Buffer
Specification](https://github.com/Esri/arcgis-pbf/tree/main/proto/FeatureCollection).

This package is built with extendr.

It is under active and very messy development.

The dream is that its fast as heck and can be used in `{arcgislayers}`.

For development, there are a number of `pbf` files in `inst/pbfs` that
can be used. I recommend using `pnts.pbf` and `small-polys.pbf` for
developmnet.

Currently the processing of polygons is very slow and over allocates.
This is probably due to an unecessary number of collections and
conversions to extendr R types.

Point processing is very fast. There is no linestring processing at the
moment. The goal is to actually avoid using `geo-types` because of the
lack of support for z and m values. The package should just go straight
to sf objects.

Alternatively, it may make sense to write to a geoarrow array and
convert to sfc using {wk}. These are just thoughts.
