use esripbf::feature_collection_p_buffer::{FeatureResult, Value, FieldType};
use extendr_api::prelude::*;
use crate::field_type_robj_mapper;

pub fn process_table(x: FeatureResult) -> Robj {

    let n = x.features.len();
    let n_fields = x.fields.len();

    
    let field_types = x
        .fields
        .iter()
        .map(|fi| fi.field_type())
        .collect::<Vec<FieldType>>();

    let field_names = x
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
    let feats = x.features;

    // iterate through features and push into attr_vecs
    // should do the same for coordinates during this iteration but not at the moment
    feats
        .into_iter()
        .for_each(|xi| {
            let atrs = xi.attributes;
            atrs.into_iter()
                .enumerate()
                .for_each(|(i, ai)| attr_vecs[i].push(ai));
        });

    let res_vecs = attr_vecs
        .into_iter()
        .zip(field_types.iter())
        .map(|(vi, fi)| {
            let field_parser = field_type_robj_mapper(fi);
            field_parser(vi)
        })
        .collect::<Vec<Robj>>();

    let row_index = (1..=n).map(|i| i as i32).collect::<Vec<i32>>();

    List::from_names_and_values(field_names, res_vecs)
        .unwrap()
        .set_attrib("row.names", row_index)
        .unwrap()
        .set_class(&["data.frame"])
        .unwrap()
}