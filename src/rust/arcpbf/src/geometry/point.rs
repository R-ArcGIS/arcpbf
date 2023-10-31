use extendr_api::prelude::*;

use esripbf::feature_collection_p_buffer::{feature::CompressedGeometry, Scale, Translate};

use crate::geometry::delta_decode;

// Read a single point geometry
pub fn read_point(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Robj {
    if x.is_none() {
        let empty_pnt = Doubles::from_values([Rfloat::na(); 2]);
        return empty_pnt.into();
    }

    let mut crds = match x.unwrap() {
        CompressedGeometry::Geometry(g) => g.coords,
        CompressedGeometry::ShapeBuffer(_) => todo!(),
    };

    let crds = delta_decode(&mut crds, trans, scale);
    let res = Doubles::from_values(crds[0]);
    res.into_robj().set_class(&["XY", "POINT", "sfg"]).unwrap()
}

pub fn read_multipoint(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Robj {
    if x.is_none() {
        let empty_mpnt = Doubles::new(0)
            .into_robj()
            .set_attrib("dim", Integers::from_values([0, 2]))
            .unwrap();
        return empty_mpnt;
    }

    let mut crds = match x.unwrap() {
        CompressedGeometry::Geometry(g) => g.coords,
        CompressedGeometry::ShapeBuffer(_) => todo!(),
    };

    let decoded = delta_decode(&mut crds, trans, scale);

    RMatrix::new_matrix(decoded.len(), 2, |r, c| decoded[r][c])
        .set_class(&["XY", "MULTIPOINT", "sfg"])
        .unwrap()
}
