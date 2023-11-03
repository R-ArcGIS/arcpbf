#' Extract body from response
#'
#' Responses of type `application/x-protobuf` are automatically
#' processed using `process_pbf()` with optional post-processing
#' applied.
#'
#' This function assumes the body is an Esri FeatureCollection
#' protobuf. It does not support Knowledge Graph protobufs.
#'
#' @param post_process default `TRUE`. Apply `post_process_pbf()` to the
#'   pbf body.
#' @inheritParams httr2::resp_body_raw
#' @inheritParams post_process_pbf
#' @export
#' @examples
#'
#' if (rlang::is_installed("httr2")) {
#'   url <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/ACS_Population_by_Race_and_Hispanic_Origin_Boundaries/FeatureServer/2/query?where=1=1&outFields=objectid&resultRecordCount=10&f=pbf&token="
#'
#'   req <- httr2::request(url)
#'   resp <- httr2::req_perform(req)
#'   resp_body_pbf(resp)
#' }
#' @family httr2
resp_body_pbf <- function(resp, post_process = TRUE, use_sf = TRUE) {
  if (requireNamespace("httr2", quietly = TRUE)) {

    if (httr2::resp_content_type(resp) != "application/x-protobuf") {
      rlang::abort("`resp` must have a content type of `application/x-protobuf`")
    }

    bits <- httr2::resp_body_raw(resp)
    res <- process_pbf(bits)
  } else {
    rlang::abort("`httr2` is required to process `httr2_response` objects.")
  }

  if (post_process) {
    post_process_pbf(res, use_sf)
  } else {
    res
  }
}
