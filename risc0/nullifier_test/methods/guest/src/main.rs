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
    let g2 = g1 + g1;
    let g3 = g1 + g2;
    let g4 = g1 + g3;
    let g5 = g1 + g4;
        
    let s1 = jubjub::Fr::from(87329482u64);
    let s2 = jubjub::Fr::from(37264829u64);
    let s3 = jubjub::Fr::from(98098098u64);
    let s4 = jubjub::Fr::from(63980948u64);
    let s5 = jubjub::Fr::from(15098098u64);

    let utxo = g1*s1 + g2*s2 + g3*s3 + g4*s4 + g5*s5;

    let rep_nsk = s1.to_bytes();
    let rep_utxo = utxo.to_bytes();

    //Note: Fr here is ark_bn254's and not jubjub's...
    let mut poseidon = light_poseidon::Poseidon::<Fr>::new_circom(2).unwrap();

    let _ = poseidon.hash_bytes_le(&[&rep_utxo, &rep_nsk]);
    // write public output to the journal
    env::commit(&input);
}
