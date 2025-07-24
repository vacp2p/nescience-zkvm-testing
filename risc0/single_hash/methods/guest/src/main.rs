use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};
use zerocopy::IntoBytes;
use k256::elliptic_curve::group::GroupEncoding;


fn main() {
    // read the input
    let input: u32 = env::read();

    let version = String::from("NSSA_v01");
    let owner = k256::ProjectivePoint::GENERATOR;
    let amount = 3u64;
    let storage = String::from("test_string_of_32_chars_storage_");
    let nonce = 24u64;
    let privacy = 1u8;
    let const1 = 7u64;
    let const2 = 124u64;

    //version
    //owner
    let mut array2 = [owner.to_bytes()];
    let mut array3 = [amount.try_into().unwrap()];
    //storage
    let array4 = [  nonce.try_into().unwrap(),
                    privacy,
                    const1.try_into().unwrap(),
                    const2.try_into().unwrap(),];
    //array2.clone_from_slice(&array3);
    array3.clone_from_slice(&array4);

    //array1.clone_from_slice(&array3);
    //array1.clone_from_slice(array4);
                
    let _hash = Impl::hash_bytes(&array3);

    // write public output to the journal
    env::commit(&input);
}