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

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod arcpbf;
    fn read_pbf;
    fn open_pbf;
    fn process_pbf;
}
