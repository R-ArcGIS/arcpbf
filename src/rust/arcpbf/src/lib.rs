use extendr_api::prelude::*;
mod geometry;
mod parse;
use parse::field_type_robj_mapper;

mod table;
use process::process_layer;
use table::process_table;

mod process;


use esripbf::esri_p_buffer::FeatureCollectionPBuffer;
use esripbf::feature_collection_p_buffer::{
    query_result::Results, FieldType, Value, 
    GeometryType
};
use std::io::Cursor;

use prost::Message;

#[extendr]
fn open_pbf(path: &str) -> Raw {
    let ff = std::fs::read(path).unwrap();
    let crs = Cursor::new(ff);
    Raw::from_bytes(&crs.into_inner())
}

// #[extendr]
// fn process_pbf(proto: Raw) -> Robj {
//     let bits = proto.as_slice();
//     let fc = FeatureCollectionPBuffer::decode(bits).unwrap();
//     let res = fc.query_result.unwrap().results.unwrap();
//     ().into_robj()
// }

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

    // If fr.spatial_reference is None then its a table
    // If Some() then its a feature layer
    // There should be two functions here.
    //   1. Process Tables
    //   2. Process Geometries
    //      - If Multipatch or has Z or has M error for now
    //      - they are not supported
    // 
    // transform informatoion used when processing geometry
    // need to remove unwraps probably for tables

    if fr.spatial_reference.is_none() {
        return process_table(fr);
    } else {
        return process_layer(fr);
    }

        // based on the type of input we need to assign geom_processor
    // a function to process each individaul geometry
    let geom_processor = match fr.geometry_type() {
        GeometryType::EsriGeometryTypePoint => geometry::read_point,
        GeometryType::EsriGeometryTypeMultipoint => geometry::read_multipoint,
        GeometryType::EsriGeometryTypePolyline => geometry::read_polyline,
        GeometryType::EsriGeometryTypePolygon => geometry::read_polygon,
        GeometryType::EsriGeometryTypeMultipatch => todo!(),
        GeometryType::EsriGeometryTypeNone => todo!(),
    };

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

            geom_processor(xi.compressed_geometry, &trans, &scale).into_robj()
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
    fn read_pbf;
    fn open_pbf;
}
