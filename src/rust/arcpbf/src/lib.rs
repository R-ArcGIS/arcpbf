use extendr_api::prelude::*;
mod geometry;
mod parse;
use parse::field_type_robj_mapper;
mod table;
use process::{process_counts, process_feature_result, process_oid};
mod process;
use anyhow::{anyhow, Result};
use esripbf::{
    esri_p_buffer::FeatureCollectionPBuffer, feature_collection_p_buffer::query_result::Results,
};
use prost::Message;
use std::io::Cursor;

#[extendr]
/// Read a pbf file as a raw vector
///
/// @param path the path to the `.pbf` file.
///
/// @returns a raw vector
/// @export
/// @examples
/// count_fp <- system.file("count.pbf", package = "arcpbf")
/// oid_fp <- system.file("ids.pbf", package = "arcpbf")
/// tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
/// fc_fp <- system.file("small-points.pbf", package = "arcpbf")
/// count_raw <- open_pbf(count_fp)
/// oid_raw <- open_pbf(oid_fp)
/// tbl_raw <- open_pbf(tbl_fp)
/// fc_raw <- open_pbf(fc_fp)
fn open_pbf(path: &str) -> Result<Raw> {
    let ff = std::fs::read(path).map_err(|e| anyhow!("failed to read pbf file {path}: {e}"))?;
    let crs = Cursor::new(ff);
    Ok(Raw::from_bytes(&crs.into_inner()))
}

fn process_pbf_(proto: &[u8]) -> Result<Robj> {
    let fc = FeatureCollectionPBuffer::decode(proto)
        .map_err(|e| anyhow!("failed to decode FeatureCollectionPBuffer: {e}"))?;
    let res = fc
        .query_result
        .ok_or_else(|| anyhow!("pbf is missing query_result"))?
        .results
        .ok_or_else(|| anyhow!("pbf query_result is missing results"))?;

    match res {
        Results::FeatureResult(fr) => process_feature_result(fr),
        Results::CountResult(cr) => process_counts(cr),
        Results::IdsResult(ids) => process_oid(ids),
    }
}

#[extendr]
/// Process a FeatureCollection PBF
///
/// Process a pbf from a raw vector or a list of raw vectors.
///
/// @param proto either a raw vector or a list of raw vectors containing a FeatureCollection pbf
///
/// @details
///
/// There are three types of PBF FeatureCollection responses that may be
/// returned.
///
/// ### Feature Result
///
/// In the case the PBF is a `FeatureResult` and `use_sf = FALSE`, a `data.frame`
/// is returned with the spatial reference stored in the `crs` attribute.
/// Otherwise an `sf` object is returned.
///
/// ### Count Result
///
/// The PBF can also return a count result, for example if the [query parameter](https://developers.arcgis.com/rest/services-reference/enterprise/query-feature-service-layer-.htm)
/// `returnCountOnly` is set to `true`. In this case, a scalar integer vector
/// is returned.
///
/// ### Object ID Result
///
/// In the case that the query parameter `returnIdsOnly` is `true`, a
/// `data.frame` is returned containing the object IDs and the column name
/// set to the object ID field name in the feature service.
///
/// @returns
///
/// - For count results, a scalar integer.
/// - For object ID results a `data.frame` with one column.
/// - For pbfs that contain geometries, a list of 3 elements:
///     - `attributes` is a `data.frame` of the fields of the FeatureCollection
///     - `geometry` is an sfc object _**without a computed bounding box or coordinate reference system set**_
///     - `sr` is a named list of the spatial reference of the feature collection
///
/// **Important**: Use [`post_process_pbf()`] to convert to an `sf` object with a computed bounding box and CRS.
///
/// @export
///
/// @examples
/// count_fp <- system.file("count.pbf", package = "arcpbf")
/// oid_fp <- system.file("ids.pbf", package = "arcpbf")
/// tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
/// fc_fp <- system.file("small-points.pbf", package = "arcpbf")
///
/// # count response
/// count_raw <- open_pbf(count_fp)
/// process_pbf(count_raw)
///
/// # object id response
/// oid_raw <- open_pbf(oid_fp)
/// head(process_pbf(oid_raw))
///
/// # table feature collection
/// tbl_raw <- open_pbf(tbl_fp)
/// process_pbf(tbl_raw)
///
/// # feature collection with geometry
/// fc_raw <- open_pbf(fc_fp)
/// process_pbf(fc_raw)
fn process_pbf(proto: Robj) -> Result<Robj> {
    if proto.is_raw() {
        let bits = proto
            .as_raw_slice()
            .ok_or_else(|| anyhow!("proto raw vector could not be read as bytes"))?;
        process_pbf_(bits)
    } else if proto.is_list() {
        let res_vec = proto
            .as_list()
            .ok_or_else(|| anyhow!("proto could not be read as a list"))?
            .into_iter()
            .map(|(_, bi)| {
                let bits = bi
                    .as_raw_slice()
                    .ok_or_else(|| anyhow!("list element could not be read as bytes"))?;
                process_pbf_(bits)
            })
            .collect::<Result<Vec<Robj>>>()?;

        Ok(List::from_values(res_vec).into())
    } else {
        Ok(().into())
    }
}

#[extendr]
fn read_pbf_(path: &str) -> Result<Robj> {
    let ff = std::fs::read(path).map_err(|e| anyhow!("failed to read pbf file {path}: {e}"))?;
    let crs = Cursor::new(ff);
    let fc = FeatureCollectionPBuffer::decode(crs)
        .map_err(|e| anyhow!("failed to decode FeatureCollectionPBuffer: {e}"))?;
    let res = fc
        .query_result
        .ok_or_else(|| anyhow!("pbf is missing query_result"))?
        .results
        .ok_or_else(|| anyhow!("pbf query_result is missing results"))?;

    // There are 3 different types of queries that we can expect:
    // Feature Query Results, ObjectID results, or FeatureCount results
    match res {
        Results::FeatureResult(fr) => process_feature_result(fr),
        Results::CountResult(cr) => process_counts(cr),
        Results::IdsResult(ids) => process_oid(ids),
    }
}

// Attempts to process a single httr2_response element.
// Returns Ok(None) for responses that are intentionally skipped (non-200,
// wrong content type, etc.) and Err(_) for malformed responses.
fn multi_resp_process_one(ri: Robj) -> Result<Option<Robj>> {
    if !ri.inherits("httr2_response") {
        return Ok(None);
    }

    let ri = ri
        .as_list()
        .ok_or_else(|| anyhow!("httr2_response could not be read as a list"))?;

    let status = ri
        .dollar("status_code")
        .map_err(|e| anyhow!("{e}"))?
        .as_integer()
        .ok_or_else(|| anyhow!("httr2_response status_code is not an integer"))?;

    if status != 200 {
        return Ok(None);
    }

    let content_type = ri
        .dollar("headers")
        .map_err(|e| anyhow!("{e}"))?
        .dollar("content-type")
        .map_err(|e| anyhow!("{e}"))?
        .as_str()
        .ok_or_else(|| anyhow!("httr2_response content-type is not a string"))?;

    if content_type != "application/x-protobuf" {
        return Ok(None);
    }

    let binding = ri.dollar("body").map_err(|e| anyhow!("{e}"))?;

    let body = binding
        .as_raw_slice()
        .ok_or_else(|| anyhow!("httr2_response body could not be read as bytes"))?;

    process_pbf_(body).map(Some)
}

#[extendr]
fn multi_resp_process_(resps: List) -> List {
    let res_vec = resps
        .into_iter()
        .map(|(_, ri)| match multi_resp_process_one(ri) {
            Ok(Some(robj)) => robj,
            Ok(None) => ().into_robj(),
            Err(e) => {
                eprintln!("Warning message:\nFailed to process response: {e}");
                ().into_robj()
            }
        })
        .collect::<Vec<_>>();

    List::from_values(res_vec)
}

// This code illustrates how we can use rayon for this
// Its about a 2x speed up but for now we're not going
// down that path
// #[derive(Debug)]
// struct SendRobj(Robj);
// unsafe impl Send for SendRobj {}
// impl From<Robj> for SendRobj {
//     fn from(value: Robj) -> Self {
//         Self(value)
//     }
// }
// impl extendr_api::ToVectorValue for SendRobj {}
// use rayon::prelude::*;
// #[extendr]
// /// @export
// fn multi_resp_process_rayon(resps: List) -> List {
//     let bit_vec = resps
//         .into_iter()
//         .map(|(_, ri)| {
//             let ri = ri.as_list()
//                 .unwrap();
//             let binding = ri.dollar("body")
//                 .unwrap();
//             let body = binding
//                 .as_raw_slice()
//                 .unwrap();
//             body.to_vec()
//         })
//         .collect::<Vec<_>>();
//     let res_vec = bit_vec
//         .into_par_iter()
//         .map(|xi| {
//             process_pbf_(xi.as_slice()).into()
//         })
//         .collect::<Vec<SendRobj>>();
//     let res = res_vec.into_iter().map(|i| i.0).collect::<Vec<_>>();
//     List::from_values(res)
// }

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod arcpbf;
    fn read_pbf_;
    fn open_pbf;
    fn process_pbf;
    fn multi_resp_process_;
}
