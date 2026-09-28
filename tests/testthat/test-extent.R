extent_pbf <- pbf_extent(-159.3, 19.5, -68.6, 64.8, 4326, count = 4186)

test_that("extent results are named bbox vectors", {
  res <- process_pbf(extent_pbf)
  expect_s3_class(res, "pbf_extent")
  expect_identical(
    unclass(res)[1:4],
    c(xmin = -159.3, ymin = 19.5, xmax = -68.6, ymax = 64.8)
  )
  expect_identical(attr(res, "count"), 4186)
  expect_identical(attr(res, "sr")$wkid, 4326L)
})

test_that("extent results without a count have an NA count", {
  res <- process_pbf(pbf_extent(0, 0, 1, 1, 4326))
  expect_identical(attr(res, "count"), NA_real_)
})

test_that("extent results post process to an sf bbox", {
  skip_if_not_installed("sf")
  res <- post_process_pbf(process_pbf(extent_pbf))
  expect_s3_class(res, "bbox")
  expect_equal(sf::st_crs(res), sf::st_crs(4326))
  expect_identical(attr(res, "count"), 4186)
})

test_that("extent results are returned as is without sf", {
  res <- post_process_pbf(process_pbf(extent_pbf), use_sf = FALSE)
  expect_s3_class(res, "pbf_extent")
})
