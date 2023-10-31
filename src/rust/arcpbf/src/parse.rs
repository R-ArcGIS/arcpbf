use esripbf::esri_p_buffer::feature_collection_p_buffer::value::ValueType;
use esripbf::feature_collection_p_buffer::{FieldType, Value};
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
        FieldType::EsriFieldTypeDate => |x| parse_big_ints(x).into_robj(),
        // FieldType::EsriFieldTypeXml => todo!(),
        FieldType::EsriFieldTypeGlobalId => |x| parse_strings(x).into_robj(),
        // FieldType::EsriFieldTypeRaster => todo!(),
        // FieldType::EsriFieldTypeBlob => todo!(),
        // FieldType::EsriFieldTypeGeometry => todo!(),
        _ => todo!(),
    }
}
