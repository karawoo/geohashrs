use extendr_api::prelude::*;
use geohash::{decode, encode, neighbor, Coord, Direction};

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

fn as_direction(dir: String) -> Direction {
    match dir.to_lowercase().as_str() {
        "n" => Direction::N,
        "ne" => Direction::NE,
        "e" => Direction::E,
        "se" => Direction::SE,
        "s" => Direction::S,
        "sw" => Direction::SW,
        "w" => Direction::W,
        "nw" => Direction::NW,
        _ => throw_r_error("Invalid direction"),
    }
}

/// Find neighbor of a geohash in the requested direction
/// @export
#[extendr]
fn gh_neighbor(geohash: Strings, direction: String) -> Strings {
    let dir = as_direction(direction);
    geohash
        .into_iter()
        .map(|x| {
            let gh_str: &str = x.as_str();
            let n = neighbor(&gh_str, dir);
            match n {
                Ok(res) => Rstr::from(res),
                Err(_) => Rstr::na(),
            }
        })
        .collect::<Strings>()
}

/// Decode a geohash
#[extendr]
fn gh_decode(geohash: Strings) -> Robj {
    let mut x_res = Vec::new();
    let mut y_res = Vec::new();
    let mut x_err = Vec::new();
    let mut y_err = Vec::new();

    for i in geohash.into_iter() {
        let decoded = decode(&i);
        match decoded {
            Ok(res) => {
                let (coord, xe, ye) = res;
                x_res.push(Rfloat::from(coord.x));
                y_res.push(Rfloat::from(coord.y));
                x_err.push(Rfloat::from(xe));
                y_err.push(Rfloat::from(ye));
            }
            Err(_) => {
                x_res.push(Rfloat::na());
                y_res.push(Rfloat::na());
                x_err.push(Rfloat::na());
                y_err.push(Rfloat::na());
            }
        }
    }

    data_frame!(
        x = Doubles::from_values(x_res),
        y = Doubles::from_values(y_res),
        x_err = Doubles::from_values(x_err),
        y_err = Doubles::from_values(y_err)
    )
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod geohashrs;
    fn gh_encode;
    fn gh_neighbor;
    fn gh_decode;
}
