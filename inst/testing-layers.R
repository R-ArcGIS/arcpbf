# Point testing feature layer
# https://analysis-1.maps.arcgis.com/home/item.html?id=433f91787ef74318aab22ce8326c8fdf#overview

# Polygon
# has date field
# https://analysis-1.maps.arcgis.com/home/item.html?id=c7dc5721f6ab42f5a515ae7f832115a9&view=list&sortOrder=desc&sortField=defaultFSOrder#overview
devtools::load_all()
fp <- "inst/pbfs/big-polys.pbf"
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
