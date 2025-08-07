// use risc0_zkvm::{Prover, Receipt};
// use hex::encode;

fn main() {
    // // Example inputs
    // let key       = [0x42u8; 32];
    // let nonce     = [0x24u8; 12];
    // let plaintext = b"Hello, RISC Zero ChaCha20 demo!";
    //
    // // 1) Create the prover with the embedded guest code
    // let mut prover = Prover::new(&GUEST_ELF, &GUEST_ID)?;
    //
    // // 2) Supply inputs
    // prover.add_input_u8_slice(&key);
    // prover.add_input_u8_slice(&nonce);
    // prover.add_input_u8_slice(plaintext);
    //
    // // 3) Run, getting a Receipt (proof + journal)
    // let receipt: Receipt = prover.run()?;
    //
    // // 4) (Optionally) verify the proof
    // receipt.verify(&GUEST_ID)?;
    //
    // // 5) Extract and print the ciphertext
    // let ct: &[u8] = receipt.get_journal_bytes();
    // println!("Ciphertext: {}", encode(ct));
    //
}

