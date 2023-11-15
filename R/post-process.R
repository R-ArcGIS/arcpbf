#' Post process pbf results
#'
#' Applies post-processing to the results of `process_pbf()`
#'
#' @details
#'
#' If `x` is a list object, the results will be row-binded. This is appropriate
#' if each element in the list is a `data.frame` or a feature result with
#' geometry. However, if each element is _not_ the same, the post-processing
#' _will_ error. If you cannot be certain that all elements that you will be
#' post processing will be the same, post-process each list element
#' independently.
#'
#' @param x an object as returned by `process_pbf()` or `read_pbf()`
#' @param use_sf default `TRUE`. Whether or not to return an `sf` object.

#' @export
#' @returns
#'
#' An object of class `data.frame`, `sf`, or a scalar integer vector.
#'
#' See [`process_pbf()`] for more details.
#'
#' @examples
#' tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
#' fc_fp <- system.file("small-points.pbf", package = "arcpbf")
#'
#' # table feature collection
#' fc <- read_pbf(tbl_fp)
#' head(post_process_pbf(fc))
#'
#' # feature collection with geometry
#' fc <- read_pbf(fc_fp)
#' head(post_process_pbf(fc))
post_process_pbf <- function(x, use_sf = TRUE) {
  if (is.data.frame(x)) {
    x
  } else if (is.list(x) && !is.null(names(x))) {
    x <- post_process_single(x, use_sf)
    if (use_sf) {
      x[[attr(x, "sf_column")]] <- sf::st_sfc(x[[attr(x, "sf_column")]])
    }
    x
  } else if (is.list(x) && is.null(names(x))) {
    post_process_list(x, use_sf)
  } else {
    x
  }
}

post_process_single <- function(x, use_sf) {

  if (is.data.frame(x)) {
    x
  } else if (use_sf && !is.data.frame(x) && is.list(x) && !is.null(names(x))) {

    rlang::check_installed("sf", "to create `sf` objects.")

    sf_crs <- crs(x[["sr"]])
    sf::st_sf(
      x[["attributes"]],
      geometry = x[["geometry"]],
      crs = sf_crs
    )
  } else if (is.list(x)) {
    sr_info <- x[["sr"]]
    x <- cbind(x[["attributes"]], x[["geometry"]])
    attr(x, "crs") <- sr_info
    x
  } else {
    x
  }
}

post_process_list <- function(x, use_sf) {
  for (i in seq_along(x)) {
    x[[i]] <- post_process_single(x[[i]], use_sf)
  }

  # check the class of the first element
  # if data.frame bind all rows
  if (inherits(x[[1]], "data.frame")) {
    x <- squish_dfs(x)

    if (use_sf && inherits(x, "sf")) {
      # force recalculation of the bounding box
      x[[attr(x, "sf_column")]] <- sf::st_sfc(x[[attr(x, "sf_column")]])
    }

    # if the first element is a numeric then
    # its a bunch of counts make it into a vector
  } else if (inherits(x[[1]], "numeric")) {
    x <- unlist(x)
  }

  x
}

# helper function to determine which component of the spatialReference needs
# to be passed to sf::st_crs() to create the spatial reference object
crs <- function(sr) {
  possible_crs <- sr[c("latest_wkid", "wkid", "wkt")]
  valid_crs_idx <- which(!is.na(possible_crs))[1]
  possible_crs[[valid_crs_idx]]
}


# squishes data frames as fast as possible
squish_dfs <- function(x) {
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
  x
}
