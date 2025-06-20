use extendr_api::prelude::*;
use geohash::{encode, Coord};

/// Print coordinate
/// @export
#[extendr]
fn gh_encode(x: Vec<f64>, y: Vec<f64>, length: usize) -> Vec<String> {
    if length < 1 || length > 12 {
        throw_r_error("`length` must be in [1, 12]")
    }
    x.into_iter()
        .zip(y.into_iter())
        .map(|(xi, yi)| {
            let coord = Coord { x: xi, y: yi };
            let encoded = encode(coord, length);
            encoded.unwrap()
        })
        .collect::<Vec<String>>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod geohashrs;
    fn gh_encode;
}
