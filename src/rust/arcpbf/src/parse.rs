use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use esripbf::esri_p_buffer::feature_collection_p_buffer::value::ValueType;
use esripbf::feature_collection_p_buffer::{FieldType, SpatialReference, Value};
use extendr_api::prelude::*;

use crate::temporal::{DateOnly, TimestampOffset};

// Treats an explicit null_value the same as an absent value
fn non_null(x: Value) -> Option<ValueType> {
    x.value_type
        .filter(|v| !matches!(v, ValueType::NullValue(_)))
}

pub fn parse_small_ints(x: Vec<Value>) -> Result<Doubles> {
    x.into_iter()
        .map(|xi| match non_null(xi) {
            Some(x) => match x {
                ValueType::SintValue(i) => Ok(Rfloat::from(i)),
                ValueType::StringValue(s) => Ok(s.parse::<f64>().map_or(Rfloat::na(), Rfloat::from)),
                ValueType::Int64Value(i) => Ok(Rfloat::from(i as f64)),
                ValueType::Sint64Value(i) => Ok(Rfloat::from(i as f64)),
                _ => {
                    bail!("Encountered unexpected value type of {x:?} please report an issue at https://github.com/R-ArcGIS/arcpbf/issues/new")
                },
            },
            None => Ok(Rfloat::na()),
        })
        .collect::<Result<Doubles>>()
}

pub fn parse_big_ints(x: Vec<Value>) -> Result<Doubles> {
    x.into_iter()
        .map(|xi| match non_null(xi) {
            Some(x) => match x {
                ValueType::Int64Value(i) => Ok(Rfloat::from(i as f64)),
                ValueType::Uint64Value(i) => Ok(Rfloat::from(i as f64)),
                ValueType::Sint64Value(i) => Ok(Rfloat::from(i as f64)),
                ValueType::UintValue(i) => Ok(Rfloat::from(i as f64)),
                _ => bail!("Encountered unexpected value type of {x:?} for a big integer field"),
            },
            None => Ok(Rfloat::na()),
        })
        .collect::<Result<Doubles>>()
}

pub fn parse_floats(x: Vec<Value>) -> Result<Doubles> {
    x.into_iter()
        .map(|xi| match non_null(xi) {
            Some(x) => match x {
                ValueType::FloatValue(f) => Ok(Rfloat::from(f as f64)),
                ValueType::DoubleValue(f) => Ok(Rfloat::from(f)),
                _ => bail!("Encountered unexpected value type of {x:?} for a float field"),
            },
            None => Ok(Rfloat::na()),
        })
        .collect::<Result<Doubles>>()
}

pub fn parse_strings(x: Vec<Value>) -> Result<Strings> {
    x.into_iter()
        .map(|xi| match non_null(xi) {
            Some(x) => match x {
                ValueType::StringValue(xx) => Ok(Rstr::from(xx)),
                _ => bail!("Encountered unexpected value type of {x:?} for a string field"),
            },
            None => Ok(Rstr::na()),
        })
        .collect::<Result<Strings>>()
}

pub fn parse_date(x: Vec<Value>) -> Result<Robj> {
    let res = x
        .into_iter()
        .map(|xi| match non_null(xi) {
            Some(x) => match x {
                ValueType::Sint64Value(i) => Ok(Rfloat::from((i / 1000_i64) as f64)),
                _ => bail!("Encountered unexpected value type of {x:?} for a date field"),
            },
            None => Ok(Rfloat::na()),
        })
        .collect::<Result<Doubles>>()?
        .into_robj()
        .set_class(["POSIXct", "POSIXt"])
        .map_err(|e| anyhow!("{e}"))?
        .clone();
    Ok(res)
}

// Parses ISO 8601 strings into doubles, leaving unparseable strings NA
fn parse_iso_strings<T: FromStr>(x: Vec<Value>, to_double: fn(T) -> f64) -> Result<Doubles> {
    x.into_iter()
        .map(|xi| match non_null(xi) {
            Some(ValueType::StringValue(s)) => Ok(s
                .parse::<T>()
                .map_or(Rfloat::na(), |v| Rfloat::from(to_double(v)))),
            Some(x) => bail!("Encountered unexpected value type of {x:?} for an ISO 8601 field"),
            None => Ok(Rfloat::na()),
        })
        .collect::<Result<Doubles>>()
}

pub fn parse_date_only(x: Vec<Value>) -> Result<Robj> {
    let res = parse_iso_strings(x, DateOnly::days)?
        .into_robj()
        .set_class(["Date"])
        .map_err(|e| anyhow!("{e}"))?
        .clone();
    Ok(res)
}

pub fn parse_timestamp_offset(x: Vec<Value>) -> Result<Robj> {
    let res = parse_iso_strings(x, TimestampOffset::seconds)?
        .into_robj()
        .set_class(["POSIXct", "POSIXt"])
        .map_err(|e| anyhow!("{e}"))?
        .clone();
    Ok(res)
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
    let latest_wkid = if x.lastest_wkid == 0 {
        Rint::na()
    } else {
        Rint::from(x.lastest_wkid as i32)
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

pub fn parse_blob(x: Vec<Value>) -> Robj {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(v) => match v {
                ValueType::NullValue(_) => ().into_robj(),
                ValueType::StringValue(v) => v.into_robj(),
                ValueType::FloatValue(v) => v.into_robj(),
                ValueType::DoubleValue(v) => v.into_robj(),
                ValueType::SintValue(v) => v.into_robj(),
                ValueType::UintValue(v) => v.into_robj(),
                ValueType::Int64Value(v) => v.into_robj(),
                ValueType::Uint64Value(v) => v.into_robj(),
                ValueType::Sint64Value(v) => v.into_robj(),
                ValueType::BoolValue(v) => v.into_robj(),
            },
            None => ().into_robj(),
        })
        .collect::<List>()
        .into()
}

// map field type to parser
pub fn field_type_robj_mapper(fi: &FieldType) -> fn(Vec<Value>) -> Result<Robj> {
    match fi {
        FieldType::EsriFieldTypeSmallInteger => |x| Ok(parse_small_ints(x)?.into_robj()),
        FieldType::EsriFieldTypeInteger => |x| Ok(parse_small_ints(x)?.into_robj()),
        FieldType::EsriFieldTypeSingle => |x| Ok(parse_floats(x)?.into_robj()),
        FieldType::EsriFieldTypeDouble => |x| Ok(parse_floats(x)?.into_robj()),
        FieldType::EsriFieldTypeString => |x| Ok(parse_strings(x)?.into_robj()),
        FieldType::EsriFieldTypeGuid => |x| Ok(parse_strings(x)?.into_robj()),
        FieldType::EsriFieldTypeOid => |x| Ok(parse_big_ints(x)?.into_robj()),
        FieldType::EsriFieldTypeDate => |x| parse_date(x),
        FieldType::EsriFieldTypeGlobalId => |x| Ok(parse_strings(x)?.into_robj()),
        FieldType::EsriFieldTypeBlob => |x| Ok(parse_blob(x)),
        FieldType::EsriFieldTypeXml => |x| Ok(parse_strings(x)?.into_robj()),
        FieldType::EsriFieldTypeBigInteger => |x| Ok(parse_big_ints(x)?.into_robj()),
        FieldType::EsriFieldTypeDateOnly => |x| parse_date_only(x),
        FieldType::EsriFieldTypeTimeOnly => |x| Ok(parse_strings(x)?.into_robj()),
        FieldType::EsriFieldTypeTimestampOffset => |x| parse_timestamp_offset(x),

        _ => |x| {
            eprintln!("This field type is not supported.\nPlease report an issue at https://github.com/R-ArcGIS/arcpbf/issues\nProvide the FeatureService URL if possible");
            Ok(List::new(x.len()).into_robj())
        },
    }
}

#[cfg(test)]
mod tests;
