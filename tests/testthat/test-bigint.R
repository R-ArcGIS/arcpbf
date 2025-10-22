test_that("parsing big integers works", {
  resp <- "https://services2.arcgis.com/FiaPA4ga0iQKduv3/ArcGIS/rest/services/Colleges_and_Universities_View/FeatureServer/0" |>
    httr2::request() |>
    httr2::req_body_form(
      f = "pbf",
      outSR = '{"wkid":3857}',
      outFields = "*",
      where = "1=1",
      resultRecordOffset = 10
    ) |>
    httr2::req_perform()

  process_pbf(resp$body)
})

# $returnGeometry
# [1] TRUE

# $outSR
# {"wkid":3857}
# $outFields
# [1] "*"

# $where
# [1] "1=1"

# $f
# [1] "pbf"
