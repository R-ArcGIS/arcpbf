use crate::parse::parse_spatial_ref;
use crate::table::process_table;
use crate::{field_type_robj_mapper, geometry};
use extendr_api::prelude::*;

use esripbf::feature_collection_p_buffer::{
    CountResult, FeatureResult, FieldType, GeometryType, ObjectIdsResult, Value,
};

pub fn process_layer(fr: FeatureResult) -> Robj {
    // get nrow and ncol here
    let n = fr.features.len();
    let n_fields = fr.fields.len();

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

    let sfc_class = match fr.geometry_type() {
        GeometryType::EsriGeometryTypePoint => ["sfc_POINT", "sfc"],
        GeometryType::EsriGeometryTypeMultipoint => ["sfc_MULTIPOINT", "sfc"],
        GeometryType::EsriGeometryTypePolyline => ["sfc_MULTILINESTRING", "sfc"],
        GeometryType::EsriGeometryTypePolygon => ["sfc_POLYGON", "sfc"],
        _ => unreachable!(),
    };

    // extract the spatial reference
    // it needs to be returned as a list object for construction
    // in sfc object
    let sr = fr.spatial_reference.unwrap();

    let sr_list = parse_spatial_ref(sr);

    let transform = fr.transform.unwrap_or_default();
    let trans = transform.clone().translate.unwrap_or_default();
    let scale = transform.scale.unwrap_or_default();

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

    // we create an empty bounding box to assign to the sfc class object
    // after processing, we will calculate the bbox
    let empty_bbox = Doubles::from_values([Rfloat::na(), Rfloat::na(), Rfloat::na(), Rfloat::na()])
        .into_robj()
        .set_names(&["xmin", "ymin", "xmax", "ymax"])
        .unwrap()
        .clone();

    // recreate the NA CRS. This will be populated later
    let na_crs = list!(
        input = Strings::from(Rstr::na()),
        wkt = Strings::from(Rstr::na())
    )
    .set_class(["crs"])
    .unwrap()
    .clone()
    .into_robj();

    let geoms = geoms
        .into_robj()
        .set_class(sfc_class)
        .unwrap()
        .set_attrib("precision", 0f64)
        .unwrap()
        .set_attrib("n_empty", 0i32)
        .unwrap()
        .set_attrib("bbox", empty_bbox)
        .unwrap()
        .set_attrib("crs", na_crs)
        .unwrap()
        .clone();

    // iterate over the
    let res_vecs = attr_vecs
        .into_iter()
        .zip(field_types.iter())
        .map(|(vi, fi)| {
            let field_parser = field_type_robj_mapper(fi);
            field_parser(vi)
        })
        .collect::<Vec<Robj>>();

    let row_index = (1..=n).map(|i| i as i32).collect::<Vec<i32>>();

    let attr_df = List::from_names_and_values(field_names, res_vecs)
        .unwrap()
        .set_attrib("row.names", row_index)
        .unwrap()
        .set_class(&["data.frame"])
        .unwrap()
        .clone();

    let res = list!(attributes = attr_df, geometry = geoms, sr = sr_list);

    res.into_robj()
}

pub fn process_feature_result(fr: FeatureResult) -> Robj {
    // for now we will return NULL if z or m dimensions are present
    if fr.has_m || fr.has_z {
        eprintln!("Warning message:\nZ and M dimensions are not supported at this time.");
        return ().into_robj();
    }
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
        process_layer(fr)
    }
}

pub fn process_counts(x: CountResult) -> Robj {
    Rfloat::from(x.count as f64).into_robj()
}

pub fn process_oid(x: ObjectIdsResult) -> Robj {
    let ids = x
        .object_ids
        .into_iter()
        .map(|xi| Rfloat::from(xi as f64))
        .collect::<Doubles>();

    let row_ind = (1..=ids.len())
        .map(|i| Rint::from(i as i32))
        .collect::<Integers>();

    List::from_names_and_values([x.object_id_field_name], [ids])
        .unwrap()
        .set_class(&["data.frame"])
        .unwrap()
        .set_attrib("row.names", row_ind)
        .unwrap()
        .clone()
        .into()
}
