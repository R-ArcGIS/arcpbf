test_that("dates and ints are returned", {
  skip_on_cran()
  skip_if_not_installed("arcgislayers")
  library(arcgislayers)
  res <- "https://services.arcgis.com/GL0fWlNkwysZaKeV/arcgis/rest/services/TXLA_ZCTA_PRCPpred/FeatureServer/0" |>
    arc_open() |>
    arc_select(n_max = 10)

  expect_true(inherits(res$DATE, "POSIXct"))

  furl <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/USA_Major_Cities_/FeatureServer/0"

  # create a reference to Layer
  cities_fl <- arc_open(furl)

  # Read in as an {sf} object
  res <- arc_select(cities_fl, n_max = 10)
  expect_true(is.numeric(res$POP_CLASS))
})

test_that("empty queries return null", {
  skip_on_cran()
  skip_if_not_installed("arcgislayers")
  library(arcgislayers)
  layer <- arc_open(
    "https://services.arcgis.com/P3ePLMYs2RVChkJx/ArcGIS/rest/services/USA_Counties_Generalized_Boundaries/FeatureServer/0"
  )

  res <- arc_select(
    layer,
    where = "STATE_NAME LIKE 'Marylnd'"
  )

  expect_null(res)
})
