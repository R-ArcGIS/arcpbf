# Minimal protobuf encoders for building FeatureCollection fixtures in tests
pb_varint <- function(n) {
  out <- raw()
  repeat {
    byte <- n %% 128
    n <- n %/% 128
    if (n == 0) return(c(out, as.raw(byte)))
    out <- c(out, as.raw(byte + 128))
  }
}

pb_uint <- function(field, n) c(pb_varint(field * 8), pb_varint(n))

pb_bytes <- function(field, x) c(pb_varint(field * 8 + 2), pb_varint(length(x)), x)

pb_string <- function(field, s) pb_bytes(field, charToRaw(s))

pb_double <- function(field, x) {
  c(pb_varint(field * 8 + 1), writeBin(x, raw(), size = 8, endian = "little"))
}

pbf_value_string <- function(s) pb_string(1, s)

pbf_value_sint64 <- function(n) pb_uint(8, if (n >= 0) 2 * n else -2 * n - 1)

pbf_value_null <- function() pb_uint(10, 1)

pbf_field <- function(name, field_type) {
  pb_bytes(13, c(pb_string(1, name), pb_uint(2, field_type)))
}

pbf_feature <- function(...) {
  pb_bytes(15, unlist(lapply(list(...), function(v) pb_bytes(1, v))))
}

# A table FeatureCollection with no spatial reference
pbf_table <- function(fields, features) {
  pb_bytes(2, pb_bytes(1, c(unlist(fields), unlist(features))))
}

# An extent result, with a count only when one is given
pbf_extent <- function(xmin, ymin, xmax, ymax, wkid, count = NULL) {
  envelope <- c(
    pb_double(1, xmin),
    pb_double(2, ymin),
    pb_double(3, xmax),
    pb_double(4, ymax),
    pb_bytes(5, pb_uint(1, wkid))
  )
  counted <- if (!is.null(count)) pb_uint(2, count)
  pb_bytes(2, pb_bytes(4, c(pb_bytes(1, envelope), counted)))
}
