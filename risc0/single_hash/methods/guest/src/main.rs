use k256::elliptic_curve::group::GroupEncoding;
use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};
use zerocopy::IntoBytes;

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
    let mut bytes_to_hash: [u8; 66] = [0; 66];

    let owner_bytes: [u8; 33] = owner.to_bytes().try_into().unwrap();
    bytes_to_hash[..33].copy_from_slice(&owner_bytes);
    bytes_to_hash[33..41].copy_from_slice(&amount.to_le_bytes());
    bytes_to_hash[41..49].copy_from_slice(&nonce.to_le_bytes());
    bytes_to_hash[49..50].copy_from_slice(&privacy.to_le_bytes());
    bytes_to_hash[50..58].copy_from_slice(&const1.to_le_bytes());
    bytes_to_hash[58..].copy_from_slice(&const2.to_le_bytes());

    let _hash = Impl::hash_bytes(&bytes_to_hash);

    // write public output to the journal
    env::commit(&input);
}
