use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

fn main() {
    // read the input
    let input: u32 = env::read();


    let version = b"NSSA_v01";
    //owner_x is the x coordinate of the GENERATOR in k256.
    let owner_x = [0x79, 0xbe, 0x66, 0x7e, 0xf9, 0xdc, 0xbb, 0xac, 0x55, 0xa0, 0x62, 0x95, 0xce, 0x87,
            0x0b, 0x07, 0x02, 0x9b, 0xfc, 0xdb, 0x2d, 0xce, 0x28, 0xd9, 0x59, 0xf2, 0x81, 0x5b,
            0x16, 0xf8, 0x17, 0x98,];
    let amount = 3u64;
    let storage = b"test_string_of_32_chars_storage_";
    let nonce = 24u64;
    let privacy = 1u8;
    let const1 = 7u64;
    let const2 = 124u64;

    let h1 = Impl::hash_bytes(version);
    let h2 = Impl::hash_bytes(&owner_x);
    let h3 = Impl::hash_bytes(&[amount.try_into().unwrap()]);
    let h4 = Impl::hash_bytes(storage);
    let h5 = Impl::hash_bytes(&[nonce.try_into().unwrap()]);
    let h6 = Impl::hash_bytes(&[privacy]);
    let h7 = Impl::hash_bytes(&[const1.try_into().unwrap()]);
    let h8 = Impl::hash_bytes(&[const2.try_into().unwrap()]);

    let h11 = Impl::hash_pair(&h1, &h2);
    let h12 = Impl::hash_pair(&h3, &h4);
    let h13 = Impl::hash_pair(&h5, &h6);
    let h14 = Impl::hash_pair(&h7, &h8);

    let h21 = Impl::hash_pair(&h11, &h12);
    let h22 = Impl::hash_pair(&h13, &h14);

    let _root = Impl::hash_pair(&h21, &h22);


    // write public output to the journal
    env::commit(&input);
}