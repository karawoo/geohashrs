use extendr_api::prelude::*;
use geohash::{encode, Coord};

/// Print coordinate
/// @export
#[extendr]
fn gh_encode(x: f64, y: f64, length: usize) -> String {
    let coord = Coord { x, y };
    let encoded = encode(coord, length);
    encoded.unwrap()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod geohashrs;
    fn gh_encode;
}
