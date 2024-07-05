use esripbf::esri_p_buffer::feature_collection_p_buffer::value::ValueType;
use esripbf::feature_collection_p_buffer::{FieldType, SpatialReference, Value};
use extendr_api::prelude::*;

// Functions to parse each field type
pub fn parse_small_ints(x: Vec<Value>) -> Integers {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(x) => match x {
                ValueType::SintValue(i) => Rint::from(i),
                _ => unreachable!(),
            },
            None => Rint::na(),
        })
        .collect::<Integers>()
}

pub fn parse_big_ints(x: Vec<Value>) -> Doubles {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(x) => match x {
                ValueType::Int64Value(i) => Rfloat::from(i as f64),
                ValueType::Uint64Value(i) => Rfloat::from(i as f64),
                ValueType::Sint64Value(i) => Rfloat::from(i as f64),
                ValueType::UintValue(i) => Rfloat::from(i as f64),
                _ => unreachable!(),
            },
            None => Rfloat::na(),
        })
        .collect::<Doubles>()
}

pub fn parse_floats(x: Vec<Value>) -> Doubles {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(x) => match x {
                ValueType::FloatValue(f) => Rfloat::from(f as f64),
                ValueType::DoubleValue(f) => Rfloat::from(f),
                _ => unreachable!(),
            },
            None => Rfloat::na(),
        })
        .collect::<Doubles>()
}

pub fn parse_strings(x: Vec<Value>) -> Strings {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(x) => match x {
                ValueType::StringValue(xx) => Rstr::from(xx),
                _ => unreachable!(),
            },
            None => Rstr::na(),
        })
        .collect::<Strings>()
}

pub fn parse_date(x: Vec<Value>) -> Robj {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(x) => match x {
                ValueType::Sint64Value(i) => Rfloat::from((i / 1000_i64) as f64),
                _ => unreachable!(),
            },
            None => Rfloat::na(),
        })
        .collect::<Doubles>()
        .into_robj()
        .set_class(["POSIXct", "POSIXt"])
        .unwrap()
        .clone()
}

pub fn parse_spatial_ref(x: SpatialReference) -> List {
    let wkt = if x.wkt.len() == 0 {
        Strings::from(Rstr::na())
    } else {
        Strings::from(Rstr::from(x.wkt))
    };
    let wkid = if x.wkid == 0 {
        Rint::na()
    } else {
        Rint::from(x.wkid as i32)
    };
    let latest_wkid = if x.latest_wkid == 0 {
        Rint::na()
    } else {
        Rint::from(x.latest_wkid as i32)
    };
    let vcs_wkid = if x.vcs_wkid == 0 {
        Rint::na()
    } else {
        Rint::from(x.vcs_wkid as i32)
    };
    let latest_vcs_wkid = if x.latest_vcs_wkid == 0 {
        Rint::na()
    } else {
        Rint::from(x.latest_vcs_wkid as i32)
    };

    list!(
        wkt = wkt,
        wkid = wkid,
        latest_wkid = latest_wkid,
        vcs_wkid = vcs_wkid,
        latest_vcs_wkid = latest_vcs_wkid
    )
}

// map field type to parser
pub fn field_type_robj_mapper(fi: &FieldType) -> fn(Vec<Value>) -> Robj {
    match fi {
        FieldType::EsriFieldTypeSmallInteger => |x| parse_small_ints(x).into_robj(),
        FieldType::EsriFieldTypeInteger => |x| parse_small_ints(x).into_robj(),
        FieldType::EsriFieldTypeSingle => |x| parse_floats(x).into_robj(),
        FieldType::EsriFieldTypeDouble => |x| parse_floats(x).into_robj(),
        FieldType::EsriFieldTypeString => |x| parse_strings(x).into_robj(),
        FieldType::EsriFieldTypeGuid => |x| parse_strings(x).into_robj(),
        FieldType::EsriFieldTypeOid => |x| parse_big_ints(x).into_robj(),
        FieldType::EsriFieldTypeDate => |x| parse_date(x),
        // FieldType::EsriFieldTypeXml => todo!(),
        FieldType::EsriFieldTypeGlobalId => |x| parse_strings(x).into_robj(),
        // FieldType::EsriFieldTypeRaster => todo!(),
        // FieldType::EsriFieldTypeBlob => todo!(),
        // FieldType::EsriFieldTypeGeometry => todo!(),
        _ => todo!(),
    }
}
