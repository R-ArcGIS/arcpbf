#' Post process pbf results
#'
#'
#'
#' @param x an object as returned by `process_pbf()` or `read_pbf()`
#' @param use_sf default `TRUE`. Whether or not to return an `sf` object.
#'   If `FALSE`, a `data.frame` is returned with the spatial reference stored
#'   in the `crs` attribute.
#' @returns
#' An object of class `data.frame` or `sf` when `use_sf = TRUE`.
#' @export
post_process_pbf <- function(x, use_sf = TRUE) {
  if (is.list(x) && !is.null(names(x))) {
    post_process_single(x, use_sf)
  } else if (is.data.frame(x)) {
    x
  } else if (is.list(x) && is.null(names(x))) {
    post_process_list(x, use_sf)
  }
}

post_process_single <- function(x, use_sf) {
  if (use_sf) {
    if (!requireNamespace("sf")) {
      stop("`sf` is required to post-process as sf obejcts")
    }
    sf_crs <- crs(x[["sr"]])
    sf::st_sf(
      x[["attributes"]],
      geometry = x[["geometry"]],
      crs = sf_crs
    )
  } else {
    sr_info <- x[["sr"]]
    x <- cbind(x[["attributes"]], x[["geometry"]])
    attr(x, "crs") <- sr_info
    x
  }
}

post_process_list <- function(x, use_sf) {
  for (i in seq_along(x)) {
    x[[i]] <- post_process_single(x[[i]], use_sf)
  }

  if (requireNamespace("data.table")) {
    x <- as.data.frame(data.table::rbindlist(x))
  } else if (requireNamespace("dplyr")) {
    x <- dplyr::bind_rows(x)
  } else {
    x <- do.call(rbind.data.frame, x)
  }

  if (use_sf) {
    sf::st_as_sf(x)
  } else {
    x
  }
}

crs <- function(sr) {
  possible_crs <- sr[c("latest_wkid", "wkid", "wkt")]
  valid_crs_idx <- which(!is.na(possible_crs))[1]
  possible_crs[[valid_crs_idx]]
}
