use risc0_zkvm::guest::env;
use group::{Group, GroupEncoding};
use ark_bn254::Fr;
use light_poseidon::PoseidonBytesHasher;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let g1 = jubjub::SubgroupPoint::generator();
        
    let s1 = jubjub::Fr::from(87329482u64);

    let rep_nsk = s1.to_bytes();
    let rep_utxo = g1.to_bytes();

    //Note: Fr here is ark_bn254's and not jubjub's...
    let mut poseidon = light_poseidon::Poseidon::<Fr>::new_circom(2).unwrap();

    let _ = poseidon.hash_bytes_le(&[&rep_utxo, &rep_nsk]);
    // write public output to the journal
    env::commit(&input);
}
