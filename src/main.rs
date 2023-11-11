use std::env::current_dir;

use nova_scotia::{circom::reader::load_r1cs, FileLocation};

fn main() {
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();

    let circuit_file = root.join("");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file = root.join("");

    let input = vec![
        116, 101, 115, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];

    let expected_output = vec![
        37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88, 212,
        4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
    ];
}
