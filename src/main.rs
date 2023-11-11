use std::env::current_dir;

use nova_scotia::{circom::reader::load_r1cs, FileLocation};
use serde::{Deserialize, Serialize};

fn main() {
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();

    let circuit_file = root.join("examples/bitcoin/circom/bitcoin_benchmark.r1cs");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file =
        root.join("examples/bitcoin/circom/bitcoin_benchmark_cpp/bitcoin_benchmark");
}
