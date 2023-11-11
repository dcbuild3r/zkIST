pub mod utils;
use nova_snark::PublicParams;
use pasta_curves::Fq;
use serde_json::{json, Value};
use std::{collections::HashMap, env::current_dir, time::Instant};
use utils::bytes_to_bitstring;

use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F,
};

fn main() {
    type G1 = pasta_curves::pallas::Point;
    type G2 = pasta_curves::vesta::Point;

    let root = current_dir().unwrap();

    let circuit_file = root.join("build/keccak32.r1cs");
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file = root.join("build/keccak32_cpp/keccak32");

    let input: Vec<u8> = vec![
        116, 101, 115, 116];

    let public_inputs: Vec<Fq> = Vec::new();

    let input_bitstring: String = bytes_to_bitstring(input);

    let mut private_inputs: Vec<HashMap<String, Value>> = Vec::new();

    let mut private_input: HashMap<String, serde_json::Value> = HashMap::new();

    private_input.insert("nBitsIn".to_string(), json!(input_bitstring));

    let expected_output: Vec<u8> = vec![
        37, 17, 98, 135, 161, 178, 88, 97, 125, 150, 143, 65, 228, 211, 170, 133, 153, 9, 88, 212,
        4, 212, 175, 238, 249, 210, 214, 116, 170, 85, 45, 21,
    ];

    let expected_output_bitstring: String = bytes_to_bitstring(expected_output);

    private_input.insert("nBitsOut".to_string(), json!(expected_output_bitstring));

    private_inputs.push(private_input);

    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());

    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file),
        r1cs,
        private_inputs.clone(),
        public_inputs.clone(),
        &pp,
    )
    .unwrap();

    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(
        &pp,
        private_inputs.len(),
        &public_inputs,
        &[F::<G2>::zero()],
    );
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    let verifier_time = start.elapsed();
    assert!(res.is_ok());
}
