mock_resp <- function(body) {
  resp <- list(
    method = "POST",
    url = "https://fake.com",
    status_code = 200L,
    headers = list(
      `content-type` = "application/x-protobuf",
      `content-length` = "8699",
      date = "Wed, 15 Nov 2023 16:23:08 GMT",
      `cache-control` = "public, max-age=30, s-maxage=30",
      `content-encoding` = "gzip",
      etag = "sd110054_1895910304",
      `access-control-allow-origin` = "*",
      `content-disposition` = "inline;filename=results.pbf"
    ) |>
      structure(class = "httr2_headers"),
    request = list(
      url = "https://services2.arcgis.com/j80Jz20at6Bi0thr/ArcGIS/rest/services/List_of_Providers/FeatureServer/27/query",
      method = NULL,
      headers = list(),
      body = list(
        data = list(
          outFields = I("%2A"),
          where = I("1%3D1"),
          returnGeometry = I("TRUE"),
          token = I(""),
          f = I("pbf"),
          resultOffset = I("0")
        ),
        type = "form",
        content_type = "application/x-www-form-urlencoded",
        params = list()
      ),
      fields = list(),
      options = list(),
      policies = list()
    ) |>
      structure(class = "httr2_request"),
    cache = environment()
  ) |>
    structure(class = "httr2_response")

  resp[["body"]] <- body

  resp
}

# WITH POST PROCESSING
test_that("post process of response tables", {

  skip_on_cran()
  skip_if_not_installed(c("httr2", "sf"))

  tbl_fp <- system.file("small-table.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)

  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps))

})

test_that("post process list of feature classes", {

  skip_on_cran()
  skip_if_not_installed(c("httr2", "sf"))

  tbl_fp <- system.file("small-points.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)


  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps))

})

test_that("post process list of OIDs", {

  skip_on_cran()
  skip_if_not_installed(c("httr2"))

  tbl_fp <- system.file("ids.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)


  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps))

})


test_that("post process list of counts", {

  skip_on_cran()
  skip_if_not_installed(c("httr2"))

  tbl_fp <- system.file("count.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)

  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps))

})


# WITHOUT POST PROCESSING
test_that("DO NOT post process of response tables", {

  skip_on_cran()
  skip_if_not_installed(c("httr2", "sf"))

  tbl_fp <- system.file("small-table.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)

  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps, FALSE))

})

test_that("DO NOT post process list of feature classes", {

  skip_on_cran()
  skip_if_not_installed(c("httr2", "sf"))

  tbl_fp <- system.file("small-points.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)

  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps, FALSE))

})

test_that("DO NOT post process list of OIDs", {

  skip_on_cran()
  skip_if_not_installed(c("httr2"))

  tbl_fp <- system.file("ids.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)


  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps, FALSE))

})

test_that("DO NOT post process list of counts", {

  skip_on_cran()
  skip_if_not_installed(c("httr2"))

  tbl_fp <- system.file("count.pbf", package = "arcpbf")

  body <- open_pbf(tbl_fp)
  resp <- mock_resp(body)

  resps <- list(resp, resp, resp)
  expect_snapshot(resps_data_pbf(resps))

})
