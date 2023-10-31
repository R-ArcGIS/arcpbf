pub mod poly;
pub use poly::{read_polygon, read_polyline};

pub mod point;
pub use point::*;


use esripbf::esri_p_buffer::feature_collection_p_buffer::{
    Scale, Translate,
};


// Delta decodes an integer vector mutably 
// Return a vector of length 2 arrays 
fn delta_decode(x: &mut [i64], trans: &Translate, scale: &Scale) -> Vec<[f64;2]> {
    for i in 2..x.len() {
        x[i] = x[i - 2] + x[i]
    }
    
    let res = x
        .chunks(2)
        .into_iter()
        .map(|c| {
            let x = c[0] as f64 * scale.x_scale + trans.x_translate;
            // ((y * scaley) - transy) * -1
            let y = c[1] as f64;
            let y = ((y * scale.y_scale) - trans.y_translate) * -1f64;
            [x,y]
        })
        .collect::<Vec<[f64; 2]>>();

    res
}