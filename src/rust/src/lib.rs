use extendr_api::prelude::*;
use geohash::{encode, Coord};

/// Print coordinate
/// @export
#[extendr]
fn gh_encode(x: Doubles, y: Doubles, length: usize) -> Vec<Rstr> {
    if length < 1 || length > 12 {
        throw_r_error("`length` must be in [1, 12]")
    }
    x.into_iter()
        .zip(y.into_iter())
        .map(|(xi, yi)| {
            let is_missing = xi.is_na() || yi.is_na();
            match is_missing {
                true => Rstr::na(),
                false => {
                    let coord = Coord {
                        x: xi.inner(),
                        y: yi.inner(),
                    };
                    let encoded = encode(coord, length);
                    match encoded {
                        Ok(encoded) => Rstr::from(encoded),
                        Err(_) => Rstr::na(),
                    }
                }
            }
        })
        .collect::<Vec<Rstr>>()
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod geohashrs;
    fn gh_encode;
}
