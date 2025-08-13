#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec; 
use alloc::vec::Vec;
use risc0_zkvm::guest::env;
use hkdf::Hkdf;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sha3::{digest::{ExtendableOutput, Update, XofReader}, Shake256};
//use serde_big_array::BigArray;

risc0_zkvm::guest::entry!(main);


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


#[derive(Debug, Serialize, Deserialize)]
pub struct EncInput {
    pub plaintext: Vec<u8>,
    pub ss_bytes: [u8; 32],
    pub epk_bytes: Vec<u8>,
    pub ipk_bytes: Vec<u8>,
    pub commitment: [u8; 32],
    pub out_index: u32,
}

fn nssa_kdf(
    ss_bytes: &[u8; 32],
    epk: &[u8],
    ipk: &[u8],
    commitment: &[u8; 32],
    out_index: u32,
) -> ([u8; 32], Vec<u8>) {
    // salt = SHA256("NSSA/v0.1/KDF-SHA256")
    let mut hasher = Sha256::new();
    sha2::Digest::update(&mut hasher, b"NSSA/v0.1/KDF-SHA256");
    let salt = hasher.finalize();

    let hk = Hkdf::<Sha256>::new(Some(&salt), ss_bytes);

    // info = "NSSA/v0.1/enc" || Epk || Ipk || commitment || le(out_index)
    let mut info = Vec::with_capacity(3 + 33 + 33 + 32 + 4 + 16);
    info.extend_from_slice(b"NSSA/v0.1/enc");
    info.extend_from_slice(epk);
    info.extend_from_slice(ipk);
    info.extend_from_slice(commitment);
    info.extend_from_slice(&out_index.to_le_bytes());

    let mut k_enc = [0u8; 32];
    hk.expand(&info, &mut k_enc).unwrap();

    (k_enc, info)
}

fn enc_xor_shake256(k_enc: &[u8; 32], ad: &[u8], pt: &[u8]) -> Vec<u8> {
    let mut shake = Shake256::default();
    shake.update(b"NSSA/v0.1/ENC/SHAKE256");
    shake.update(k_enc);
    shake.update(ad);
    let mut xof = shake.finalize_xof();

    let mut ks = vec![0u8; pt.len()];
    xof.read(&mut ks);

    pt.iter().zip(ks).map(|(p, k)| p ^ k).collect()
}

pub fn main() {
    let input: EncInput = env::read();

    let (k_enc, info) = nssa_kdf(
        &input.ss_bytes,
        &input.epk_bytes,
        &input.ipk_bytes,
        &input.commitment,
        input.out_index,
    );

    let ct = enc_xor_shake256(&k_enc, &info, &input.plaintext);

    // Commit ciphertext to the journal
    env::commit_slice(&ct);
}