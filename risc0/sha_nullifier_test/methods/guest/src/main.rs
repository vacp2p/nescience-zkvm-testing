use risc0_zkvm::{
    guest::env,
    sha::{Impl, Sha256}
};

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let s0 = String::from("test_string_0 for nullifiers in NSSA");

    let test_bytes0 = s0.as_bytes();
 
    let _ = Impl::hash_bytes(test_bytes0);   
  
     // write public output to the journal
    env::commit(&input);
}
