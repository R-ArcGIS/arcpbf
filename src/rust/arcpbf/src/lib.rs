use extendr_api::prelude::*;
mod geometry;
mod parse;
use geometry::*;
use parse::field_type_robj_mapper;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

use esripbf::esri_p_buffer::FeatureCollectionPBuffer;
use esripbf::feature_collection_p_buffer::{query_result::Results, FieldType, Value};
use std::io::Cursor;

use prost::Message;

#[extendr]
fn open_pbf(path: &str) -> Raw {
    let ff = std::fs::read(path).unwrap();
    let crs = Cursor::new(ff);
    Raw::from_bytes(&crs.into_inner())
}

#[extendr]
fn read_pbf(path: &str) -> Robj {
    let ff = std::fs::read(path).unwrap();
    let crs = Cursor::new(ff);
    let fc = FeatureCollectionPBuffer::decode(crs).unwrap();
    let res = fc.query_result.unwrap().results.unwrap();

    // There are 3 different types of queries that we can expect:
    // Feature Query Results, ObjectID results, or FeatureCount results
    let fr = if let Results::FeatureResult(r) = res {
        r
    } else {
        todo!()
    };
    
    let n = fr.features.len();
    let n_fields = fr.fields.len();

    // transform informatoion used when processing geometry
    // need to remove unwraps probably for tables
    let transform = fr.transform.unwrap();
    let trans = transform.clone().translate.unwrap();
    let scale = transform.scale.unwrap();

    // TODO return spatial reference information for {sf} to convert
    // let sr = fr.spatial_reference.unwrap();

    let field_types = fr
        .fields
        .iter()
        .map(|fi| fi.field_type())
        .collect::<Vec<FieldType>>();

    let field_names = fr
        .fields
        .into_iter()
        .map(|xi| xi.name)
        .collect::<Vec<String>>();

    // pre-allocate vectors to store attributes
    // let mut attr_vecs: Vec<Vec<Option<ValueType>>> = Vec::with_capacity(n_fields);
    let mut attr_vecs: Vec<Vec<Value>> = (0..n_fields)
        .map(|_| Vec::with_capacity(n))
        .collect::<Vec<_>>();

    // access geometries and attributes separately
    let feats = fr.features;

    // iterate through features and push into attr_vecs
    // should do the same for coordinates during this iteration but not at the moment
    let geoms = feats
        .into_iter()
        .map(|xi| {
            let atrs = xi.attributes;
            atrs.into_iter()
                .enumerate()
                .for_each(|(i, ai)| attr_vecs[i].push(ai));

            read_multipoint(xi.compressed_geometry, &trans, &scale).into_robj()
        })
        .collect::<Vec<Robj>>();

    // iterate over the
    let res_vecs = attr_vecs
        .into_iter()
        .zip(field_types.iter())
        .map(|(vi, fi)| {
            let field_parser = field_type_robj_mapper(fi);
            field_parser(vi)
        })
        .collect::<Vec<Robj>>();

    let attr_res = List::from_values(res_vecs).set_names(field_names).unwrap();

    list!(geometry = geoms, attributes = attr_res).into_robj()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod arcpbf;
    fn hello_world;
    fn read_pbf;
    fn open_pbf;
}
