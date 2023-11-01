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
/// @export
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
/// @export
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
/// 
/// @export
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
