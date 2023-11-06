#' Read a FeatureCollection Protocol Buffer
#'
#' Given a binary file containing a FeatureCollection protocol buffer (pbf),
#' read its contents into R as an R object.
#'
#' @param path a scalar character of the path to the pbf file
#' @inheritParams resp_body_pbf
#'
#' @returns
#'
#' Either a data.frame, list, scalar integer, or sf object if
#' `post_process = TRUE` and `use_sf = TRUE`.
#'
#' See [`process_pbf()`] for more.
#'
#' @examples
#' count_fp <- system.file("count.pbf", package = "arcpbf")
#' oid_fp <- system.file("ids.pbf", package = "arcpbf")
#' tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
#' fc_fp <- system.file("small-points.pbf", package = "arcpbf")
#'
#' # count response
#' read_pbf(count_fp)
#'
#' # object id response
#' head(read_pbf(oid_fp))
#'
#' # table feature collection
#' read_pbf(tbl_fp)
#'
#' # feature collection with geometry
#' read_pbf(fc_fp)
#'
#' @export
read_pbf <- function(path, post_process = TRUE, use_sf = TRUE) {
  bits <- read_pbf_(path)

  if (post_process) {
    post_process_pbf(bits, use_sf)
  } else {
    bits
  }
}






