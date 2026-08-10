use anyhow::{anyhow, bail, Result};
use extendr_api::prelude::*;

use esripbf::feature_collection_p_buffer::{feature::CompressedGeometry, Scale, Translate};

use crate::geometry::delta_decode;

// Read a single point geometry
pub fn read_point(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Result<Robj> {
    let mut crds = match x {
        None => {
            let empty_pnt = Doubles::from_values([Rfloat::na(); 2])
                .into_robj()
                .set_class(&["XY", "POINT", "sfg"])
                .map_err(|e| anyhow!("{e}"))?
                .clone();

            return Ok(empty_pnt);
        }
        Some(CompressedGeometry::Geometry(g)) => g.coords,
        Some(CompressedGeometry::ShapeBuffer(_)) => bail!("ShapeBuffer geometry is not supported"),
    };

    let crds = delta_decode(&mut crds, trans, scale);
    let res = Doubles::from_values(crds[0]);
    let res = res
        .into_robj()
        .set_class(&["XY", "POINT", "sfg"])
        .map_err(|e| anyhow!("{e}"))?
        .clone();
    Ok(res)
}

pub fn read_multipoint(
    x: Option<CompressedGeometry>,
    trans: &Translate,
    scale: &Scale,
) -> Result<Robj> {
    let mut crds = match x {
        None => {
            let empty_mpnt = Doubles::new(0)
                .into_robj()
                .set_attrib("dim", Integers::from_values([0, 2]))
                .map_err(|e| anyhow!("{e}"))?
                .set_class(&["XY", "MULTIPOINT", "sfg"])
                .map_err(|e| anyhow!("{e}"))?
                .clone();

            return Ok(empty_mpnt);
        }
        Some(CompressedGeometry::Geometry(g)) => g.coords,
        Some(CompressedGeometry::ShapeBuffer(_)) => bail!("ShapeBuffer geometry is not supported"),
    };

    let decoded = delta_decode(&mut crds, trans, scale);

    let res = RMatrix::new_matrix(decoded.len(), 2, |r, c| decoded[r][c])
        .set_class(&["XY", "MULTIPOINT", "sfg"])
        .map_err(|e| anyhow!("{e}"))?
        .clone();
    Ok(res)
}
