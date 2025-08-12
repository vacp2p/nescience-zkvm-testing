#[cfg(not(rust_analyzer))]

#[cfg(rust_analyzer)]
//mod methods {
   // pub const GUEST_ELF: &[u8] = &[];
   // pub const GUEST_ID: [u32; 8] = [0; 8];
//}
#[cfg(rust_analyzer)]
use methods::*;


use anyhow::Result; 
use risc0_zkvm::{default_prover, ExecutorEnv};

use shake256_demo::{enc_xor_shake256, nssa_kdf, EncInput};
//ßuse methods::GUEST_ELF;

use serde::{Serialize, Deserialize};
//use serde_big_array::BigArray;
use hex::ToHex; // for encode_hex
use shake256_demo::methods::GUEST_ELF; // <— crate name uses underscore

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeystreamRequest {
    pub kdf_salt: [u8; 32],
    pub ss_bytes: [u8; 32],
    pub epk_bytes: Vec<u8>,
    pub ipk_bytes: Vec<u8>,
    pub commitment: [u8; 32],
    pub out_index: u32,
    pub pt: Vec<u8>,
}

fn main() -> Result<()> {
    // Tiny demo inputs (in real NSSA you’d compute ss via ECDH).
    let plaintext = b"hello NSSA (shake256)".to_vec();
    let ss_bytes = [1u8; 32];
    let epk = vec![2u8; 33];
    let ipk = vec![3u8; 33];
    let commitment = [4u8; 32];
    let out_index = 0u32;

    let input = EncInput {
        plaintext: plaintext.clone(),
        ss_bytes,
        epk_bytes: epk.clone(),
        ipk_bytes: ipk.clone(),
        commitment,
        out_index,
    };

    // Prove inside zkVM
    let env = ExecutorEnv::builder().write(&input)?.build()?;
    let prove_info = default_prover().prove(env, GUEST_ELF)?;
    println!("WARNING: proving in dev mode. Not production-safe.");

    // Ciphertext from journal
    let guest_ct = prove_info.receipt.journal.bytes.to_vec();
    println!("guest ct: {}", guest_ct.encode_hex::<String>());

    // Host recompute (sanity)
    let (k_enc, info) = nssa_kdf(&ss_bytes, &epk, &ipk, &commitment, out_index);
    let host_ct = enc_xor_shake256(&k_enc, &info, &plaintext);
    println!("host  ct: {}", host_ct.encode_hex::<String>());

    assert_eq!(guest_ct, host_ct);
    println!("OK: guest and host ciphertexts match.");
    Ok(())
}

