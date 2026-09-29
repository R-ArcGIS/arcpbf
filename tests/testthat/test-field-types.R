fields <- list(
  pbf_field("big", 13),
  pbf_field("date_only", 14),
  pbf_field("time_only", 15),
  pbf_field("offset", 16),
  pbf_field("small", 0),
  pbf_field("xml", 12)
)

features <- list(
  pbf_feature(
    pbf_value_sint64(2^40),
    pbf_value_string("2024-02-29"),
    pbf_value_string("14:35:00.927"),
    pbf_value_string("2003-01-25T14:35:00.927-08:00"),
    pbf_value_string("abc"),
    pbf_value_string("<a/>")
  ),
  pbf_feature(
    pbf_value_null(),
    pbf_value_null(),
    pbf_value_null(),
    pbf_value_null(),
    pbf_value_string("34"),
    pbf_value_null()
  ),
  pbf_feature(
    pbf_value_sint64(-5),
    pbf_value_string("1900-03-01"),
    pbf_value_string("00:00:00"),
    pbf_value_string("1969-12-31T16:00:00-08:00"),
    pbf_value_string("2020-01-01"),
    pbf_value_string("<b/>")
  )
)

res <- post_process_pbf(process_pbf(pbf_table(fields, features)))

test_that("big integer fields are numeric", {
  expect_identical(res$big, c(2^40, NA, -5))
})

test_that("date-only fields are Dates", {
  expect_s3_class(res$date_only, "Date")
  expect_equal(res$date_only, as.Date(c("2024-02-29", NA, "1900-03-01")))
})

test_that("time-only fields are hms", {
  expect_s3_class(res$time_only, "hms")
  expect_equal(as.numeric(res$time_only), c(52500.927, NA, 0))
})

test_that("timestamp offset fields are UTC POSIXct", {
  expect_s3_class(res$offset, "POSIXct")
  expect_identical(attr(res$offset, "tzone"), "UTC")
  expect_equal(as.numeric(res$offset), c(1043534100.927, NA, 0))
})

test_that("small integer fields holding date strings are not dates (#211)", {
  expect_false(inherits(res$small, "POSIXct"))
  expect_identical(res$small, c(NA, 34L, NA))
})

test_that("xml fields are character", {
  expect_identical(res$xml, c("<a/>", NA, "<b/>"))
})
