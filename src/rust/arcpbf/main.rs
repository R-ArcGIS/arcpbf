use esripbf::{*, feature_collection_p_buffer::{FeatureResult, feature::CompressedGeometry, value::ValueType}};
use prost::Message;
use std::io::Cursor;

use geo_types::Coord;

use esripbf::feature_collection_p_buffer::query_result::Results;
use esri_p_buffer::feature_collection_p_buffer::*;

use extendr_api::prelude::*;

fn main() -> std::io::Result<()> {
    let ff = std::fs::read("features.pbf").unwrap();
    let crs = Cursor::new(ff);
    let fc = FeatureCollectionPBuffer::decode(crs.clone()).unwrap();
    let res = fc.query_result.unwrap().results.unwrap();

    // There are 3 different types of queries that we can expect: 
    // Feature Query Results, ObjectID results, or FeatureCount results
    let fr = if let Results::FeatureResult(r) = res {
        r
    } else {
        todo!()
    };

    // preview fields
    // println!("CRS: {:#?}", fr.spatial_reference); 
    // println!("{:?}", fr.geohash_field_name);
    println!("{:#?}", fr.fields);


    let n = fr.features.len(); 
    let n_fields = fr.fields.len();

    let field_types = fr.fields.iter().map(|fi| fi.field_type())
        .collect::<Vec<FieldType>>();

    // pre-allocate vectors to store attributes
    // let mut attr_vecs: Vec<Vec<Option<ValueType>>> = Vec::with_capacity(n_fields);
    let mut attr_vecs: Vec<Vec<Value>> = (0..n_fields)
        .map(|_| Vec::with_capacity(n))
        .collect::<Vec<_>>();

    // access geometries and attributes separately
    let feats = fr.features;
    let atts = fr.values;

    // iterate through features and push into attr_vecs 
    // should do the same for coordinates during this iteration but not at the moment
    feats
        .into_iter()
        .for_each(|xi| {
            let atrs = xi.attributes;
            atrs
                .into_iter()
                .enumerate()
                .for_each(|(i, ai)| attr_vecs[i].push(ai))
        });


    // iterate over the 
    let res_vecs = attr_vecs
        .into_iter()
        .zip(field_types.iter())
        .map(|(vi, fi)| {
            let field_parser = field_type_robj_mapper(fi);
            field_parser(vi)
        }).collect::<Vec<Robj>>();

    println!("{:?}", res_vecs);

    

    // grabbed a single feature
    // let f = &feats[10].to_owned();
    // println!("{:?}", feats[10]);

    // let addtl_stride = (fr.has_m as usize) + (fr.has_z as usize);
    
    // // For a single gometry we need 
    // let crds = if let CompressedGeometry::Geometry(g) = f.clone().compressed_geometry.unwrap() {
    //     g
    // } else {
    //     todo!()
    // };


    // let transf = fr.transform.unwrap();
    // println!("{:#?}", transf);
    // let scale = transf.clone().scale.unwrap();
    // let translate = transf.translate.unwrap();

    // // this is how we can extract all of the coordinates for all features
    // let res2 = feats.into_iter()
    //     .map(|xi| {
    //         let f = xi.compressed_geometry.unwrap();
            
    //         match f {
    //             CompressedGeometry::Geometry(f) => transform_coords(f.coords, &translate, &scale),
    //             CompressedGeometry::ShapeBuffer(_) => vec![]
    //         }
    //     })
    //     .collect::<Vec<Vec<Coord>>>();

    // println!("{:?}", res2);

    

    Ok(())
}

use esripbf::feature_collection_p_buffer::Value;

fn field_type_robj_mapper(fi: &FieldType) -> fn(Vec<Value>) -> Robj {
    match fi {
        FieldType::EsriFieldTypeSmallInteger => |x| parse_small_ints(x).into_robj(),
        FieldType::EsriFieldTypeInteger => |x| parse_small_ints(x).into_robj(),
        FieldType::EsriFieldTypeSingle => |x| parse_floats(x).into_robj(),
        FieldType::EsriFieldTypeDouble => |x| parse_floats(x).into_robj(),
        FieldType::EsriFieldTypeString => |x| parse_strings(x).into_robj(),
        FieldType::EsriFieldTypeGuid => |x| parse_strings(x).into_robj(),
        FieldType::EsriFieldTypeOid => |x| parse_big_ints(x).into_robj(),
        FieldType::EsriFieldTypeDate => |x| parse_big_ints(x).into_robj(),
        _ => todo!()
        // FieldType::EsriFieldTypeXml => todo!(),
        // FieldType::EsriFieldTypeRaster => todo!(),
        // FieldType::EsriFieldTypeBlob => todo!(),
        // FieldType::EsriFieldTypeGlobalId => todo!(),
        // FieldType::EsriFieldTypeGeometry => todo!(),
    }
}




fn esri_date_to_posixct(x: Rfloat) -> Robj {
    let res = (x / 1000f64);
    res
        .into_robj()
        .set_class(["POSIXct","POSIXt"])
        .unwrap()
}

use esri_p_buffer::feature_collection_p_buffer::{Scale, Translate};

// Transforms a full vector of coordinates
fn transform_coords(input: Vec<i64>, trans: &Translate, scale: &Scale) -> Vec<Coord> {
    decode_delta_2d(input)
        .into_iter()
        .map(|xi| transform_coord(xi, &trans, &scale))
        .collect::<Vec<Coord>>() 
}

// Transforms a single coordinate
fn transform_coord(input: [i64; 2], trans: &Translate, scale: &Scale) -> Coord {
    let x = input[0] as f64 * scale.x_scale + trans.x_translate;
    let y = trans.y_translate - input[1] as f64 * scale.y_scale;
    Coord::from([x, y])
}


fn decode_delta_2d(mut x: Vec<i64>) -> Vec<[i64; 2]> {
    let init_x = x[0];
    let init_y = x[1];

    x
        .iter_mut()
        .enumerate()
        .skip(2)
        .map(|(i, value)| *value += if i % 2 == 0 { init_x } else { init_y })
        .for_each(drop);
    
    x.chunks(2)
        .into_iter()
        .map(|c| {  
            [c[0], c[1]]
        })
        .collect::<Vec<[i64;2]>>()
}

// Ideally: 
// Return list(geometry, list(attributes), spatialReference)

// Need pbf files for: 
// Polyline
// Point & Multipoint
// Z dimensions
// M dimension
// Z & M dimensions
// Date, Blob, XML, etc.