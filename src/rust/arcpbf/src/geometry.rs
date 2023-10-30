
use extendr_api::prelude::*;

extendr_module! {
    mod geometry;
}


use esripbf::esri_p_buffer::feature_collection_p_buffer::{
    Scale, Translate,
    feature::CompressedGeometry
};

// Read a single point geometry
pub fn read_point(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Robj {

    if x.is_none() {
        let empty_pnt = Doubles::from_values([Rfloat::na(); 2]);
        return empty_pnt.into();
    } 

    let crds = match x.unwrap() {
        CompressedGeometry::Geometry(g) => g.coords,
        CompressedGeometry::ShapeBuffer(_) => todo!(),
    };

    let crds = decode_delta_2d(crds);
    let res = transform_coord2d(crds[0], trans, scale);
    res
        .into_robj()
        .set_class(&["XY", "POINT", "sfg"])
        .unwrap()

}

// THIS IS VERY INEFFICIENT AND SLOW IM DOING SOMETHING BAD HERE!!  
pub fn read_poly(x: Option<CompressedGeometry>, trans: &Translate, scale: &Scale) -> Robj {

    let geoms = match x.unwrap() {
        CompressedGeometry::Geometry(g) => g,
        CompressedGeometry::ShapeBuffer(_) => todo!(),
    };

    let lens = geoms.lengths;
    let mut crds = geoms.coords;
    
    let partitions = lens.into_iter()
        .map(|li| {
            let r = 0..((li as usize) * 2);
            let part = crds.drain(r).collect();
            decode_delta_2d(part)
        })
        .collect::<Vec<Vec<[i64;2]>>>();

    let res_vec = partitions
        .into_iter()
        .map(|xi| {

            // Collect the transformed coordinates into a Vec<Doubles>
            // these need to go into a Matrix 
            let res_coords = xi.into_iter().map(|ci| {
                transform_coord2d(ci, trans, scale)
            })
            .collect::<Vec<Doubles>>();

            let res = RMatrix::new_matrix(
                res_coords.len(), 2, |r, c| res_coords[r][c]
            ).into_robj();

            res

        })
        .collect::<Vec<Robj>>();

    List::from_values(res_vec)
        .set_class(&["XY", "POLYGON", "sfg"])
        .unwrap()
}

// Transforms a single coordinate
// Takes the result of decode_delta_2d to adjust the coordinate to be float values
// in real coordinates
fn transform_coord2d(input: [i64; 2], trans: &Translate, scale: &Scale) -> Doubles {
    let x = input[0] as f64 * scale.x_scale + trans.x_translate;
    let y = input[1] as f64 * scale.y_scale + trans.y_translate;
    Doubles::from_values([x, y])
}


// Takes a delta encoded vector and undoes the delta encoding
// returns a vec of i64 which should be transformed and scaled.
fn decode_delta_2d(mut x: Vec<i64>) -> Vec<[i64; 2]> {

    for i in 2..x.len() {
        x[i] = x[i-2] + x[i]
    }

    let res = x.chunks(2)
        .into_iter()
        .map(|c| {  
            [c[0], c[1]]
        })
        .collect::<Vec<[i64;2]>>();

    res
}