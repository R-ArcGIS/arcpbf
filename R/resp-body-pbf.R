#' Extract PBFs from httr2_response objects
#'
#' Processes `httr2_response` objects that return FeatureCollection PBFs.
#'
#' @details
#'
#' Responses of type `application/x-protobuf` are automatically processed using
#'`process_pbf()` with optional post-processing applied. Theses functions
#' assume that the body of the responses are an Esri FeatureCollection
#' protocol buffer.
#'
#' ### Lists of responses
#'
#' When running multiple requests in parallel using
#' [`httr2::req_perform_parallel()`] the responses are returned as a list of
#' responses. `resps_data_pbf()` processes the responses in a vectorized
#' manner.
#'
#' Results are post-processed by default and return sf objects if applicable.
#' This may not be desirable if heterogeneous response types are expected. For
#' example, if one list element contains a count result and another contains
#' an object ID result.
#'
#' See [`post_process_pbf()`] for more details.
#'
#' Note: Knowledge Graph protocol buffers and other protobuf formats are not supported
#' and will result in an error if used with these functions.
#'
#' @param resps a list of `httr2_response` objects such as
#'   created by `httr2::req_perform_parallel()`
#'
#' @param post_process default `TRUE`. Apply `post_process_pbf()` to the
#'   pbf body.
#'
#' @inheritParams httr2::resp_body_raw
#' @inheritParams post_process_pbf
#'
#' @returns
#'
#' A processed FeatureCollection pbf. Either a scalar integer, named list,
#' data.frame, or an sf object if post-processing is applied.
#'
#' @export
#' @examples
#'
#' if (rlang::is_installed(c("httr2", "sf")) && interactive()) {
#'   base_url <- file.path(
#'     "https://services.arcgis.com/P3ePLMYs2RVChkJx",
#'     "arcgis", "rest", "services",
#'     "ACS_Population_by_Race_and_Hispanic_Origin_Boundaries",
#'     "FeatureServer", "2", "query", fsep = "/"
#'   )
#'
#'   # create the base request
#'   req <- httr2::request(base_url)
#'
#'   # fill query parameters
#'   req <- httr2::req_url_query(
#'     req,
#'     where = "1=1",
#'     outFeilds = "objectid",
#'     resultRecordCount = 1,
#'     f = "pbf"
#'   )
#'
#'   # make the request
#'   resp <- httr2::req_perform(req)
#'
#'   # parse the request
#'   resp_body_pbf(resp)
#'
#'   # simulate response from multi_req_perform
#'   resps <- list(resp, resp, resp)
#'
#'   # process them all at once
#'   resps_data_pbf(resps)
#' }
#' @rdname httr2
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



#' @export
#' @rdname httr2
resps_data_pbf <- function(resps, post_process = TRUE, use_sf = TRUE) {
  res <- multi_resp_process_(resps)

  if (post_process) {
    post_process_pbf(res, use_sf)
  } else {
    res
  }
}
