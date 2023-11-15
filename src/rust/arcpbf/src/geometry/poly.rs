use extendr_api::prelude::*;

use esripbf::esri_p_buffer::feature_collection_p_buffer::{
    feature::CompressedGeometry, Scale, Translate,
};

use crate::geometry::delta_decode;
use core::ops::Range;

// This is a function that is shared between Polygons and Polylines
// The class assignment is handled and delegated by read_polyline() and read_polygon()
// Processes a scalar geometry feature
// TODO do not unwrap `x`
pub fn read_poly(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> List {

    // if none return an empty list
    if x.is_none() {
        return list!();
    }
    let geoms = match x.unwrap() {
        CompressedGeometry::Geometry(g) => g,
        CompressedGeometry::ShapeBuffer(_) => todo!(),
    };

    let lens = geoms.lengths;
    let mut crds = geoms.coords;

    // chat gpt one here.
    let ranges: Vec<Range<usize>> = lens
        .iter()
        .scan(0, |state, &len| {
            let start = *state;
            *state += (len as usize) * 2;
            Some(start..*state)
        })
        .collect();

    let partitions = ranges
        .into_iter()
        .map(|li| {
            // let r = 0..((li as usize) * 2);
            let part = &mut crds[li];
            let decoded = delta_decode(part, trans, scale);

            RMatrix::new_matrix(decoded.len(), 2, |r, c| decoded[r][c])
        })
        .collect::<Vec<_>>();

    List::from_values(partitions)
}

pub fn read_polygon(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Robj {
    read_poly(x, trans, scale)
        .set_class(&["XY", "POLYGON", "sfg"])
        .unwrap()
}

pub fn read_polyline(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Robj {
    read_poly(x, trans, scale)
        .set_class(&["XY", "MULTILINESTRING", "sfg"])
        .unwrap()
}
