use extendr_api::prelude::*;
mod geometry;
mod parse;
use parse::field_type_robj_mapper;

mod table;
use process::{process_feature_result, process_oid, process_counts};

mod process;


use esripbf::esri_p_buffer::FeatureCollectionPBuffer;
use esripbf::feature_collection_p_buffer::{
    query_result::Results
};
use std::io::Cursor;

use prost::Message;

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
fn open_pbf(path: &str) -> Raw {
    let ff = std::fs::read(path).unwrap();
    let crs = Cursor::new(ff);
    Raw::from_bytes(&crs.into_inner())
}

fn process_pbf_(proto: &[u8]) -> Robj {
    // let bits = proto.as_slice();
    let fc = FeatureCollectionPBuffer::decode(proto).unwrap();
    let res = fc.query_result.unwrap().results.unwrap();

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
/// **Important**: Use [`post_process_pbf()`] to convet to an `sf` object with a computed bounding box and CRS.
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
fn process_pbf(proto: Robj) -> Robj {

    if proto.is_raw() {
        process_pbf_(proto.as_raw_slice().unwrap())
    } else if proto.is_list() {
        let res_vec = proto.as_list()
        .unwrap()
        .into_iter()
        .map(|(_, bi)| {
            let bits = bi.as_raw_slice().unwrap();
            process_pbf_(bits)
        })
        .collect::<Vec<Robj>>();

        List::from_values(res_vec).into()
    } else {
        ().into()
    }
    
}

#[extendr]
/// Read a FeatureCollection Protocol Buffer 
/// 
/// Given a binary file containing a FeatureCollection protocol buffer (pbf),
/// read its contents into R as an R object. 
/// 
/// @param path a scalar character of the path to the pbf file
/// 
/// @returns 
/// 
/// Either a data.frame, list, or scalar integer. 
/// 
/// See [`process_pbf()`] for more.
/// 
/// @examples 
///
/// count_fp <- system.file("count.pbf", package = "arcpbf")
/// oid_fp <- system.file("ids.pbf", package = "arcpbf")
/// tbl_fp <- system.file("small-table.pbf", package = "arcpbf")
/// fc_fp <- system.file("small-points.pbf", package = "arcpbf")
/// 
/// # count response
/// read_pbf(count_fp)
/// 
/// # object id response
/// head(read_pbf(oid_fp))
/// 
/// # table feature collection
/// read_pbf(tbl_fp)
/// 
/// # feature collection with geometry 
/// read_pbf(fc_fp)
/// 
/// @export
fn read_pbf(path: &str) -> Robj {
    let ff = std::fs::read(path).unwrap();
    let crs = Cursor::new(ff);
    let fc = FeatureCollectionPBuffer::decode(crs).unwrap();
    let res = fc.query_result.unwrap().results.unwrap();

    // There are 3 different types of queries that we can expect:
    // Feature Query Results, ObjectID results, or FeatureCount results
    match res {
        Results::FeatureResult(fr) => process_feature_result(fr),
        Results::CountResult(cr) => process_counts(cr),
        Results::IdsResult(ids) => process_oid(ids),
    }
   
}

#[extendr]
/// Process a list of httr2 responses
/// 
/// When running multiple requests in parallel using [`httr2::multi_req_perform()`]
/// the responses are returned as a list of responses. `multi_resp_process()` processes
/// the responses in a vectorized manner.
/// 
/// @details
/// 
/// If a response is not 200 status code or does not have the appropriate `"application/x-protobuf"`
/// content type, the result will be `NULL`.
/// 
/// The results of this _are not_ post processed. Post processing can be
/// applied to the resultant list object using `post_process_pbf()`. 
/// See the example for a fully worked example including post-processing.
/// 
/// @param resps a list of `httr2_response` objects such as 
///   created by `httr2::multi_req_perform()`
/// @export
/// @family httr2
/// @returns
/// A list where each element is a processed FeatureCollection PBF.
/// 
/// @examples 
/// if (rlang::is_installed("httr2") && interactive()) {
///     url <- "https://services.arcgis.com/P3ePLMYs2RVChkJx/arcgis/rest/services/ACS_Population_by_Race_and_Hispanic_Origin_Boundaries/FeatureServer/2/query?where=1=1&outFields=*&f=pbf&token="
///     reqs <- replicate(5, httr2::request(url), simplify = FALSE)
///     resps <- httr2::multi_req_perform(reqs)
///     pbfs <- multi_resp_process(resps)
///     post_process_pbf(pbfs)
/// }
fn multi_resp_process(resps: List) -> List {
    let res_vec = resps 
        .into_iter()
        .map(|(_, ri)| {

            if !ri.inherits("httr2_response") {
                return ().into_robj()
            } 

            let ri = ri.as_list()
                .unwrap();

            let status = ri.dollar("status_code")
                .unwrap()
                .as_integer()
                .unwrap();

            if status != 200 {
                return ().into_robj()
            }

            let content_type = ri
                .dollar("headers")
                .unwrap()
                .dollar("content-type")
                .unwrap()
                .as_str()
                .unwrap();

            if content_type != "application/x-protobuf" {
                return ().into_robj()
            }

            let binding = ri.dollar("body")
                .unwrap();

            let body = binding
                .as_raw_slice()
                .unwrap();

            process_pbf_(body)

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
    fn read_pbf;
    fn open_pbf;
    fn process_pbf;
    fn multi_resp_process;
    // fn multi_resp_process_rayon;
}
