use super::*;

const MS_PER_DAY: i64 = 86_400_000;

#[test]
fn dates() {
    assert_eq!("1970-01-01".parse(), Ok(DateOnly(0)));
    assert_eq!("2003-01-25".parse(), Ok(DateOnly(12_077)));
    assert_eq!("1969-12-31".parse(), Ok(DateOnly(-1)));
    assert_eq!("1900-03-01".parse(), Ok(DateOnly(-25_508)));
    assert_eq!("2024-02-29".parse(), Ok(DateOnly(19_782)));
    assert_eq!("2000-02-29".parse(), Ok(DateOnly(11_016)));
}

#[test]
fn invalid_dates() {
    assert!("2023-02-29".parse::<DateOnly>().is_err());
    assert!("1900-02-29".parse::<DateOnly>().is_err());
    assert!("2024-13-01".parse::<DateOnly>().is_err());
    assert!("yesterday".parse::<DateOnly>().is_err());
    assert!("".parse::<DateOnly>().is_err());
}

// The documented example, 2003-01-25T14:35:00.927 at -08:00, is 22:35:00.927 UTC
#[test]
fn offset_timestamps() {
    let utc = 12_077 * MS_PER_DAY + ((22 * 60 + 35) * 60) * 1000 + 927;
    assert_eq!(
        "2003-01-25T14:35:00.927-08:00".parse(),
        Ok(TimestampOffset(utc))
    );
    assert_eq!("2003-01-25T22:35:00.927Z".parse(), Ok(TimestampOffset(utc)));
    assert_eq!(
        "2003-01-26T04:05:00.927+05:30".parse(),
        Ok(TimestampOffset(utc))
    );
    assert_eq!("2003-01-25T22:35:00.927".parse(), Ok(TimestampOffset(utc)));
    assert_eq!(TimestampOffset(utc).seconds(), 1_043_534_100.927);
}

#[test]
fn offset_timestamps_across_days() {
    assert_eq!(
        "2024-02-29T23:00:00-02:00".parse(),
        Ok(TimestampOffset(19_783 * MS_PER_DAY + MS_PER_DAY / 24))
    );
    assert_eq!("1969-12-31T16:00:00-08:00".parse(), Ok(TimestampOffset(0)));
    assert_eq!(
        "1960-06-15T00:00:00Z".parse(),
        Ok(TimestampOffset(-3_487 * MS_PER_DAY))
    );
}

#[test]
fn invalid_offset_timestamps() {
    assert!("2003-01-25".parse::<TimestampOffset>().is_err());
    assert!("14:35:00".parse::<TimestampOffset>().is_err());
    assert!("2003-02-30T00:00:00Z".parse::<TimestampOffset>().is_err());
}
