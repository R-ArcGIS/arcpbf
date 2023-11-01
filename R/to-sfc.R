

to_sfc <- function(x, sr) {
  possible_crs <- sr[c("latest_wkid", "wkid", "wkt")]
  valid_crs_idx <- which(!is.na(possible_crs))[1]
  chosen_crs <- possible_crs[[valid_crs_idx]]

  sf::st_sfc(x, crs = chosen_crs)
}

# x <- read_pbf("inst/pbfs/big-multipoint.pbf")
# sr <- x$sr
# to_sfc(x$geometry, sr)
