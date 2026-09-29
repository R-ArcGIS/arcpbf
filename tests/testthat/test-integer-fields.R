test_that("integer fields are not classed as dates (#211)", {
  skip_on_cran()

  res <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/MTBS_Polygons_v1/FeatureServer/0" |>
    httr2::request() |>
    httr2::req_url_path_append("query") |>
    httr2::req_body_form(
      f = "pbf",
      outFields = "Acres,StartDate",
      where = "1=1",
      returnGeometry = "false",
      resultRecordCount = "50"
    ) |>
    httr2::req_perform() |>
    (\(x) post_process_pbf(process_pbf(x$body), use_sf = FALSE))()


  expect_false(inherits(res$Acres, "POSIXct"))
  expect_type(res$Acres, "integer")
  expect_s3_class(res$StartDate, "POSIXct")
})

test_that("integer field values survive intact (#211)", {
  skip_on_cran()

  res <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/MTBS_Polygons_v1/FeatureServer/0" |>
    httr2::request() |>
    httr2::req_url_path_append("query") |>
    httr2::req_body_form(
      f = "pbf",
      outFields = "Acres",
      where = "Acres > 0",
      returnGeometry = "false",
      resultRecordCount = "50"
    ) |>
    httr2::req_perform() |>
    (\(x) post_process_pbf(process_pbf(x$body), use_sf = FALSE))()


  expect_true(all(res$Acres > 0))
  expect_false(anyNA(res$Acres))
})
