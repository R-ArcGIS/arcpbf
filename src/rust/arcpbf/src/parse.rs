// use std::cell::OnceCell;
use once_cell::sync::OnceCell;

use anyhow::{anyhow, bail, Result};
use chrono::NaiveDateTime;
use esripbf::esri_p_buffer::feature_collection_p_buffer::value::ValueType;
use esripbf::feature_collection_p_buffer::{FieldType, SpatialReference, Value};
use extendr_api::prelude::*;

// Functions to parse each field type
pub fn parse_small_ints(x: Vec<Value>) -> Result<Doubles> {
    let is_date: OnceCell<bool> = OnceCell::new();
    // println!("starting once_cell val {:?}", is_date);
    let mut res_vec = x
        .into_iter()
        .map(|xi| match xi.value_type {
            Some(x) => match x {
                ValueType::SintValue(i) => Ok(Rfloat::from(i)),
                ValueType::StringValue(s) => {
                    let _ = is_date.set(true);
                    let maybe_date = chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d");
                    match maybe_date {
                        Ok(d) => Ok(Rfloat::from(NaiveDateTime::from(d).and_utc().timestamp() as i32)),

                        Err(_) => Ok(Rfloat::na()),
                    }
                },
                ValueType::Int64Value(i) => Ok(Rfloat::from(i as f64)),
                ValueType::Sint64Value(i) => Ok(Rfloat::from(i as f64)),
                _ => {
                    bail!("Encountered unexpected value type of {x:?} please report an issue at https://github.com/R-ArcGIS/arcpbf/issues/new")
                },
            },
            None => Ok(Rfloat::na()),
        })
        .collect::<Result<Doubles>>()?;

    // rprintln!("{:?}", is_date);
    if is_date.get().is_some_and(|x| *x) {
        let date_res = res_vec
            .set_class(["POSIXct", "POSIXt"])
            .map_err(|e| anyhow!("{e}"))?
            .clone();
        return Ok(date_res);
    }
    Ok(res_vec)
}

pub fn parse_big_ints(x: Vec<Value>) -> Result<Doubles> {
    x.into_iter()
        .map(|xi| match xi.value_type {
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
        .map(|xi| match xi.value_type {
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
        .map(|xi| match xi.value_type {
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
        .map(|xi| match xi.value_type {
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

pub fn parse_blob(x: Vec<Value>) -> Robj {
    x.into_iter()
        .map(|xi| match xi.value_type {
            Some(v) => match v {
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

        _ => |x| {
            eprintln!("This field type is not supported.\nPlease report an issue at https://github.com/R-ArcGIS/arcpbf/issues\nProvide the FeatureService URL if possible");
            Ok(List::new(x.len()).into_robj())
        },
    }
}
