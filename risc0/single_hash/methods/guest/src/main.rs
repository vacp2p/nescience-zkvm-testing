use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

fn main() {
    // read the input
    let input: u32 = env::read();

    let version = String::from("NSSA_v01");
    //owner_x is the x coordinate of the GENERATOR in k256.
    let owner_x = [0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
            0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b,
            0x16, 0xf8, 0x17, 0x98,];
    //let owner = k256::ProjectivePoint::GENERATOR;
    let amount = 3u64;
    let storage = String::from("test_string_of_32_chars_storage_");
    let nonce = 24u64;
    let privacy = 1u8;
    let const1 = 7u64;
    let const2 = 124u64;

    let mut bytes_to_hash: [u8; 105] = [0; 105];

   // let owner_bytes = owner.to_bytes();

    bytes_to_hash[..8].copy_from_slice(&version.as_bytes());
    bytes_to_hash[8..40].copy_from_slice(&owner_x);
    bytes_to_hash[40..48].copy_from_slice(&amount.to_le_bytes());
    bytes_to_hash[48..80].copy_from_slice(&storage.as_bytes());
    bytes_to_hash[80..88].copy_from_slice(&nonce.to_le_bytes());
    bytes_to_hash[88..89].copy_from_slice(&privacy.to_le_bytes());
    bytes_to_hash[89..97].copy_from_slice(&const1.to_le_bytes());
    bytes_to_hash[97..].copy_from_slice(&const2.to_le_bytes());
                
    let _hash = Impl::hash_bytes(&bytes_to_hash);

    // write public output to the journal
    env::commit(&input);
}