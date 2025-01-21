use catan_tui::{hex::Hex, number::Number, terrain::Terrain};

fn main() {
    let hex = Hex {
        terrain: Terrain::Desert,
        number: Number::_2
    };
    println!("{}", hex);
    let hex = Hex {
        terrain: Terrain::Fields,
        number: Number::_2
    };
    println!("{}", hex);
    let hex = Hex {
        terrain: Terrain::Forest,
        number: Number::_2
    };
    println!("{}", hex);
    let hex = Hex {
        terrain: Terrain::Hills,
        number: Number::_2
    };
    println!("{}", hex);
    let hex = Hex {
        terrain: Terrain::Mountains,
        number: Number::_2
    };
    println!("{}", hex);
    let hex = Hex {
        terrain: Terrain::Pasture,
        number: Number::_2
    };
    println!("{}", hex);
}
