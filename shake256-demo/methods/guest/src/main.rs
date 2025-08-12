#![no_main]
#![no_std]

extern crate alloc;

use risc0_zkvm::guest::env;

mod shared {
    extern crate alloc;
    use alloc::vec::Vec;
    use serde::{Deserialize, Serialize};
    use sha3::{
        digest::{ExtendableOutput, Update, XofReader},
        Shake256,
    };

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EncInput {
        pub plaintext: Vec<u8>,
        pub k_enc: [u8; 32],
        pub info: Vec<u8>,
    }

    pub fn enc_xor_shake256(k_enc: &[u8; 32], info: &[u8], pt: &[u8]) -> Vec<u8> {
        let mut h = Shake256::default();
        h.update(k_enc);
        h.update(info);
        let mut xof = h.finalize_xof();

        let mut ks = alloc::vec![0u8; pt.len()];
        xof.read(&mut ks);

        pt.iter().zip(ks).map(|(p, k)| p ^ k).collect()
    }
}

use shared::{enc_xor_shake256, EncInput};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let EncInput { plaintext, k_enc, info } = env::read();
    let ct = enc_xor_shake256(&k_enc, &info, &plaintext);
    env::commit(&ct);
}
