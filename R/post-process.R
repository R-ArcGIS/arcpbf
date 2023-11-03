#' Post process pbf results
#'
#' Applies post-processing to the results of `process_pbf()`
#'
#' @param x an object as returned by `process_pbf()` or `read_pbf()`
#' @param use_sf default `TRUE`. Whether or not to return an `sf` object.
#' @returns
#' An object of class `data.frame` or `sf` when `use_sf = TRUE`.
#' @export
#' @returns
#'
#' An object of class `data.frame`, `sf`, or a scalar integer vector.
#'
#' See [`process_pbf()`] for more details.
#'
#' @examples
#' count_fp <- system.file("count.pbf", package = "arcpbf")
#' oid_fp <- system.file("ids.pbf", package = "arcpbf")
#' tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
#' fc_fp <- system.file("small-points.pbf", package = "arcpbf")
#'
#' # count response
#' read_pbf(count_fp) |>
#'   post_process_pbf()
#'
#' # object id response
#' read_pbf(oid_fp) |>
#'   post_process_pbf()
#'
#' # table feature collection
#' read_pbf(tbl_fp) |>
#'   post_process_pbf()
#'
#' # feature collection with geometry
#' read_pbf(fc_fp) |>
#'   post_process_pbf()
post_process_pbf <- function(x, use_sf = TRUE) {
  if (is.data.frame(x)) {
    x
  } else if (is.list(x) && !is.null(names(x))) {
    post_process_single(x, use_sf)
  } else if (is.list(x) && is.null(names(x))) {
    post_process_list(x, use_sf)
  } else {
    x
  }
}

post_process_single <- function(x, use_sf) {
  if (use_sf) {

    rlang::check_installed("sf", "to create `sf` objects.")

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

  if (rlang::is_installed("collapse", version = "2.0.0")) {
    x <- collapse::rowbind(x)
  } else if (rlang::is_installed("data.table")) {
    x <- data.table::rbindlist(x)
    data.table::setDF(x)
  } else if (rlang::is_installed("dplyr")) {
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
