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

table_fields <- c(
  "OBJECTID", "Adoption_Service_Provider", "DBA", "city", "state",
  "Accreditation_or_Approval_Statu", "Accredited_Approvedto_Provide", "full_address"
)

# WITH POST PROCESSING
test_that("post process of response tables", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("small-table.pbf", package = "arcpbf")))
  res <- resps_data_pbf(list(resp, resp, resp))

  expect_s3_class(res, "data.frame")
  expect_false(inherits(res, "sf"))
  expect_identical(names(res), table_fields)
  expect_equal(res$OBJECTID, rep(1:3, 3))
  expect_identical(res$city, rep(c("Boulder", "Denver", "Washington"), 3))
  expect_true(all(is.na(res$DBA)))
})

test_that("post process list of feature classes", {
  skip_on_cran()
  skip_if_not_installed(c("httr2", "sf"))

  resp <- mock_resp(open_pbf(system.file("small-points.pbf", package = "arcpbf")))
  res <- resps_data_pbf(list(resp, resp, resp))

  expect_s3_class(res, "sf")
  expect_identical(nrow(res), 6L)
  expect_identical(res$County, rep("Hawaii County", 6))
  expect_identical(as.character(unique(sf::st_geometry_type(res))), "MULTIPOLYGON")
  expect_equal(sf::st_crs(res), sf::st_crs(3857))
})

test_that("post process list of OIDs", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("ids.pbf", package = "arcpbf")))
  res <- resps_data_pbf(list(resp, resp, resp))

  expect_s3_class(res, "data.frame")
  expect_identical(names(res), "OBJECTID")
  expect_equal(res$OBJECTID, rep(1:3, 3))
})

test_that("post process list of counts", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("count.pbf", package = "arcpbf")))

  expect_identical(resps_data_pbf(list(resp, resp, resp)), rep(3143, 3))
})

# WITHOUT POST PROCESSING
test_that("DO NOT post process of response tables", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("small-table.pbf", package = "arcpbf")))
  res <- resps_data_pbf(list(resp, resp, resp), FALSE)

  expect_s3_class(res, "nanoarrow_array_stream")
  df <- nanoarrow::convert_array_stream(res)
  expect_identical(names(df), table_fields)
  expect_identical(nrow(df), 9L)
})

test_that("DO NOT post process list of feature classes", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("small-points.pbf", package = "arcpbf")))
  res <- resps_data_pbf(list(resp, resp, resp), FALSE)

  expect_s3_class(res, "nanoarrow_array_stream")
  df <- nanoarrow::convert_array_stream(res)
  expect_identical(names(df), c("County", "geometry"))
  expect_identical(nrow(df), 6L)
  expect_s3_class(df$geometry, "geoarrow_vctr")
})

test_that("DO NOT post process list of OIDs", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("ids.pbf", package = "arcpbf")))
  res <- resps_data_pbf(list(resp, resp, resp), FALSE)

  expect_length(res, 3)
  for (oids in res) {
    expect_identical(names(oids), "OBJECTID")
    expect_equal(oids$OBJECTID, 1:3)
  }
})

test_that("DO NOT post process list of counts", {
  skip_on_cran()
  skip_if_not_installed("httr2")

  resp <- mock_resp(open_pbf(system.file("count.pbf", package = "arcpbf")))

  expect_identical(resps_data_pbf(list(resp, resp, resp), FALSE), list(3143, 3143, 3143))
})
