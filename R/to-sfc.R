to_sfc <- function(x, sr) {
  possible_crs <- sr[c("latest_wkid", "wkid", "wkt")]
  valid_crs_idx <- which(!is.na(possible_crs))[1]
  chosen_crs <- possible_crs[[valid_crs_idx]]

  sf::st_sfc(x, crs = chosen_crs)
}

crs <- function(sr) {
  possible_crs <- sr[c("latest_wkid", "wkid", "wkt")]
  valid_crs_idx <- which(!is.na(possible_crs))[1]
  possible_crs[[valid_crs_idx]]
}

to_sf <- function(x) {
  sfc <- to_sfc(x[["geometry"]], x[["sr"]])

  sf::st_sf(x[["attributes"]], geometry = sfc)
}

to_sfc2 <- function(x, sr) {
  structure(
    x,
    bbox = sfheaders::sf_bbox(x),
    class = c("sfc_MULTIPOINT", "sfc"),
    precision = 0,
    crs = sf::st_crs(crs(sr)),
    n_empty = 0
  )
}

