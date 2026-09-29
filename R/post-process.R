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
#' Feature results arrive as a `nanoarrow_array_stream`. With `use_sf = FALSE`
#' they become a `data.frame` whose `geometry` column is a `geoarrow_vctr`.
#'
#' @param x an object as returned by `process_pbf()` or `read_pbf()`
#' @param use_sf default `TRUE`. Whether or not to return an `sf` object.
#' @importFrom geoarrow as_geoarrow_vctr
#' @importFrom nanoarrow convert_array_stream
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
  if (inherits(x, "nanoarrow_array_stream")) {
    post_process_arrow(x, use_sf)
  } else if (inherits(x, "pbf_extent")) {
    post_process_extent(x, use_sf)
  } else if (is.list(x) && !is.data.frame(x)) {
    post_process_list(x, use_sf)
  } else {
    x
  }
}

post_process_arrow <- function(x, use_sf) {
  res <- convert_array_stream(x)
  if (!use_sf || is.null(res[["geometry"]])) {
    return(res)
  }

  rlang::check_installed("sf", "to create `sf` objects.")
  res[["geometry"]] <- sf::st_as_sfc(res[["geometry"]])
  sf::st_as_sf(res)
}

post_process_list <- function(x, use_sf) {
  for (i in seq_along(x)) {
    x[[i]] <- post_process_pbf(x[[i]], use_sf)
  }

  # check the class of the first element
  # if data.frame bind all rows
  if (inherits(x[[1]], "data.frame")) {
    x <- arcgisutils::rbind_results(x)

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

post_process_extent <- function(x, use_sf) {
  if (!use_sf) {
    return(x)
  }

  rlang::check_installed("sf", "to create `sf` objects.")

  sr <- attr(x, "sr")
  crs <- if (is.null(sr)) sf::NA_crs_ else arcgisutils::from_spatial_reference(sr)
  res <- sf::st_bbox(
    c(xmin = x[["xmin"]], ymin = x[["ymin"]], xmax = x[["xmax"]], ymax = x[["ymax"]]),
    crs = crs
  )
  attr(res, "count") <- attr(x, "count")
  res
}
