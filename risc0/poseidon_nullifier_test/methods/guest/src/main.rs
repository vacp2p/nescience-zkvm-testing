use risc0_zkvm::guest::env;
use ark_bn254::Fr;
use light_poseidon::PoseidonBytesHasher;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let s0 = String::from("test_string_0");
    let s1 = String::from("test_string_1");

    let test_bytes0 = s0.as_bytes();
    let test_bytes1 = s1.as_bytes();

    let mut poseidon = light_poseidon::Poseidon::<Fr>::new_circom(2).unwrap();

    let _ = poseidon.hash_bytes_le(&[&test_bytes0, &test_bytes1]);
    // write public output to the journal
    env::commit(&input);
}
