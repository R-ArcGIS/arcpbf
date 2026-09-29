# Each fixture pair is one query to a live service, as f=pbf and f=json.
skip_if_not_installed("sf")
skip_if_not_installed("yyjsonr")

json_opts <- yyjsonr::opts_read_json(
  obj_of_arrs_to_df = FALSE,
  arr_of_objs_to_df = FALSE,
  arr_of_arrs_to_matrix = FALSE
)

test_that("Z lines keep their Z values", {
  geoms <- sf::st_geometry(read_pbf(test_path("testdata", "lines_z.pbf")))
  json <- yyjsonr::read_json_file(test_path("testdata", "lines_z.json"), opts = json_opts)
  paths <- unlist(lapply(json$features, function(f) f$geometry$paths), recursive = FALSE)
  expected <- do.call(rbind, unlist(paths, recursive = FALSE))
  actual <- unname(sf::st_coordinates(geoms)[, c("X", "Y", "Z")])

  expect_identical(unique(lapply(geoms, class)), list(c("XYZ", "MULTILINESTRING", "sfg")))
  expect_identical(dim(actual), dim(expected))
  expect_lt(max(abs(actual - expected)), 1e-3)
})

test_that("M lines keep their M values", {
  geoms <- sf::st_geometry(read_pbf(test_path("testdata", "lines_m.pbf")))
  json <- yyjsonr::read_json_file(test_path("testdata", "lines_m.json"), opts = json_opts)
  paths <- unlist(lapply(json$features, function(f) f$geometry$paths), recursive = FALSE)
  expected <- do.call(rbind, unlist(paths, recursive = FALSE))
  actual <- unname(sf::st_coordinates(geoms)[, c("X", "Y", "M")])

  expect_identical(unique(lapply(geoms, class)), list(c("XYM", "MULTILINESTRING", "sfg")))
  expect_identical(dim(actual), dim(expected))
  expect_lt(max(abs(actual - expected)), 1e-3)
})

test_that("ZM lines keep their Z and M values", {
  geoms <- sf::st_geometry(read_pbf(test_path("testdata", "lines_zm.pbf")))
  json <- yyjsonr::read_json_file(test_path("testdata", "lines_zm.json"), opts = json_opts)
  paths <- unlist(lapply(json$features, function(f) f$geometry$paths), recursive = FALSE)
  expected <- do.call(rbind, unlist(paths, recursive = FALSE))
  actual <- unname(sf::st_coordinates(geoms)[, c("X", "Y", "Z", "M")])

  expect_identical(unique(lapply(geoms, class)), list(c("XYZM", "MULTILINESTRING", "sfg")))
  expect_identical(dim(actual), dim(expected))
  expect_lt(max(abs(actual - expected)), 1e-3)
})
