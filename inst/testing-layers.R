# Point testing feature layer
# https://analysis-1.maps.arcgis.com/home/item.html?id=433f91787ef74318aab22ce8326c8fdf#overview

# Polygon
# has date field
# https://analysis-1.maps.arcgis.com/home/item.html?id=c7dc5721f6ab42f5a515ae7f832115a9&view=list&sortOrder=desc&sortField=defaultFSOrder#overview

# Line has Z and M
# https://analysis-1.maps.arcgis.com/home/item.html?id=6d473f3e67c944c7af309f17fe055874&sublayer=1

# multipoint
# https://analysis-1.maps.arcgis.com/home/item.html?id=f2d507c021e14dd18ddd45a88f8d9a35&view=service#overview

devtools::load_all()
fp <- "inst/pbfs/small-lines.pbf"
x <- read_pbf(fp)
rw <- open_pbf(fp)


jsn <- sf::st_sf(x$attributes, sf::st_sfc(x$geometry)) |>
  arcgisutils::as_esri_featureset()

bench::mark(
  pbf = parse_pbf(rw),
  jsn = arcgisutils::parse_esri_json(jsn),
  check = F
)
crds
