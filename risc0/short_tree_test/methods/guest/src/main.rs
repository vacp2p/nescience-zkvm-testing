use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256},
};

fn main() {
    // read the input
    let input: u32 = env::read();

    let comp1 = 1u8;
    let comp2 = 2u8;
    let comp3 = 3u8;
    let comp4 = 4u8;
    let comp5 = 5u8;
    let comp6 = 6u8;
    let comp7 = 7u8;
    let comp8 = 0u8;

    let h1 = Impl::hash_bytes(&[comp1]);
    let h2 = Impl::hash_bytes(&[comp2]);
    let h3 = Impl::hash_bytes(&[comp3]);
    let h4 = Impl::hash_bytes(&[comp4]);
    let h5 = Impl::hash_bytes(&[comp5]);
    let h6 = Impl::hash_bytes(&[comp6]);
    let h7 = Impl::hash_bytes(&[comp7]);
    let h8 = Impl::hash_bytes(&[comp8]);

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