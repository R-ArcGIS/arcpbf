use extendr_api::prelude::*;
mod geometry;
mod parse;
use parse::field_type_robj_mapper;

mod table;
use process::{process_layer, process_feature_result, process_oid, process_counts};
use table::process_table;

mod process;


use esripbf::esri_p_buffer::FeatureCollectionPBuffer;
use esripbf::feature_collection_p_buffer::{
    query_result::Results
};
use std::io::Cursor;

use prost::Message;

#[extendr]
fn open_pbf(path: &str) -> Raw {
    let ff = std::fs::read(path).unwrap();
    let crs = Cursor::new(ff);
    Raw::from_bytes(&crs.into_inner())
}

fn process_pbf(proto: &[u8]) -> Robj {
    // let bits = proto.as_slice();
    let fc = FeatureCollectionPBuffer::decode(proto).unwrap();
    let res = fc.query_result.unwrap().results.unwrap();

    match res {
        Results::FeatureResult(fr) => process_feature_result(fr),
        Results::CountResult(cr) => process_counts(cr),
        Results::IdsResult(ids) => process_oid(ids),
    }
}

use rayon::prelude::*;

fn par_process_pbf_raw(protos: List) -> List {
    let bit_vec = protos
        .into_iter()
        .map(|(_, pi)| {
            let ri = pi.as_raw().unwrap();
            let ci = ri.as_slice().to_vec();
            ci
        })
        .collect::<Vec<_>>();

    let res_vec = bit_vec
        .into_par_iter()
        .map(|bi| {
            let pnt = unsafe { process_pbf(&bi).get() };

        })
        .collect::<Vec<_>>();

    List::from_values(res_vec)
}

#[extendr]
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
}
