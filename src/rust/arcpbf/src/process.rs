use anyhow::{anyhow, Result};
use extendr_api::prelude::*;

use esripbf::feature_collection_p_buffer::{
    CountResult, ExtentCountResult, ObjectIdsResult, SpatialReference,
};

pub fn process_counts(x: CountResult) -> Result<Robj> {
    Ok(Rfloat::from(x.count as f64).into_robj())
}

pub fn process_oid(x: ObjectIdsResult) -> Result<Robj> {
    let ids = x
        .object_ids
        .into_iter()
        .map(|xi| Rfloat::from(xi as f64))
        .collect::<Doubles>();

    let row_ind = (1..=ids.len())
        .map(|i| Rint::from(i as i32))
        .collect::<Integers>();

    let res = List::from_names_and_values([x.object_id_field_name], [ids])
        .map_err(|e| anyhow!("{e}"))?
        .set_class(&["data.frame"])
        .map_err(|e| anyhow!("{e}"))?
        .set_attrib("row.names", row_ind)
        .map_err(|e| anyhow!("{e}"))?
        .clone()
        .into();
    Ok(res)
}

// Returns the extent as a named bbox vector with `sr` and `count` attributes
pub fn process_extent(x: ExtentCountResult) -> Result<Robj> {
    let extent = x
        .extent
        .ok_or_else(|| anyhow!("ExtentCountResult is missing an extent"))?;

    let count = x.count.map_or(Rfloat::na(), |n| Rfloat::from(n as f64));

    let mut res = Doubles::from_values([extent.x_min, extent.y_min, extent.x_max, extent.y_max])
        .into_robj()
        .set_names(["xmin", "ymin", "xmax", "ymax"])
        .map_err(|e| anyhow!("{e}"))?
        .set_attrib("count", count)
        .map_err(|e| anyhow!("{e}"))?
        .set_class(["pbf_extent"])
        .map_err(|e| anyhow!("{e}"))?
        .clone();

    if let Some(sr) = extent.spatial_reference {
        res.set_attrib("sr", parse_spatial_ref(sr))
            .map_err(|e| anyhow!("{e}"))?;
    }

    Ok(res)
}

fn parse_spatial_ref(x: SpatialReference) -> List {
    let wkt = if x.wkt.is_empty() {
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
