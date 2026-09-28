use anyhow::{anyhow, Result};

use super::*;

fn value(v: ValueType) -> Value {
    Value {
        value_type: Some(v),
        index: None,
    }
}

fn string(s: &str) -> Value {
    value(ValueType::StringValue(s.to_string()))
}

fn null() -> Value {
    value(ValueType::NullValue(true))
}

// Reads a double vector with NA as None
fn doubles(x: &Robj) -> Result<Vec<Option<f64>>> {
    let vals = x
        .as_real_slice()
        .ok_or_else(|| anyhow!("expected a double vector"))?;
    Ok(vals
        .iter()
        .map(|v| Some(*v).filter(|v| !v.is_nan()))
        .collect())
}

fn parse(field_type: FieldType, x: Vec<Value>) -> Result<Robj> {
    extendr_engine::start_r();
    field_type_robj_mapper(&field_type)(x)
}

#[test]
fn big_integers() -> Result<()> {
    let res = parse(
        FieldType::EsriFieldTypeBigInteger,
        vec![
            value(ValueType::Sint64Value(1 << 40)),
            value(ValueType::Int64Value(-5)),
            null(),
        ],
    )?;
    assert_eq!(doubles(&res)?, vec![Some(2f64.powi(40)), Some(-5.0), None]);
    Ok(())
}

#[test]
fn date_only() -> Result<()> {
    let res = parse(
        FieldType::EsriFieldTypeDateOnly,
        vec![
            string("2024-02-29"),
            string("1900-03-01"),
            string("nope"),
            null(),
        ],
    )?;
    assert!(res.inherits("Date"));
    assert_eq!(
        doubles(&res)?,
        vec![Some(19_782.0), Some(-25_508.0), None, None]
    );
    Ok(())
}

#[test]
fn timestamp_offset() -> Result<()> {
    let res = parse(
        FieldType::EsriFieldTypeTimestampOffset,
        vec![string("2003-01-25T14:35:00.927-08:00"), null()],
    )?;
    assert!(res.inherits("POSIXct"));
    assert_eq!(doubles(&res)?, vec![Some(1_043_534_100.927), None]);
    Ok(())
}

#[test]
fn time_only_and_xml_are_strings() -> Result<()> {
    for field_type in [
        FieldType::EsriFieldTypeTimeOnly,
        FieldType::EsriFieldTypeXml,
    ] {
        let res = parse(field_type, vec![string("14:35:00.927")])?;
        assert_eq!(res.as_str_vector(), Some(vec!["14:35:00.927"]));
    }
    Ok(())
}

// A small integer column of date strings stays numeric (arcgislayers#211)
#[test]
fn small_integers_are_never_dates() -> Result<()> {
    let res = parse(
        FieldType::EsriFieldTypeSmallInteger,
        vec![
            string("2020-01-01"),
            string("34"),
            value(ValueType::SintValue(7)),
            null(),
        ],
    )?;
    assert!(!res.inherits("POSIXct"));
    assert_eq!(doubles(&res)?, vec![None, Some(34.0), Some(7.0), None]);
    Ok(())
}

#[test]
fn unexpected_value_types_error() {
    let res = parse(
        FieldType::EsriFieldTypeDateOnly,
        vec![value(ValueType::BoolValue(true))],
    );
    assert!(res.is_err());
}
