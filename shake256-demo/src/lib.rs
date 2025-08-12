#![no_std]

extern crate alloc;

use alloc::vec;      
use alloc::vec::Vec;
use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};
use serde::{Serialize, Deserialize};

/// Re-export the generated method symbols.
pub mod methods {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

/// Inputs the host sends to the guest.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncInput {
    pub plaintext: Vec<u8>,
    pub k_enc: [u8; 32],  // 32-byte encryption key
    pub info: Vec<u8>,    // domain-separated info bytes (same on host & guest)
}

/// XOR-encrypt using SHAKE256 keystream derived from k_enc || info.
pub fn enc_xor_shake256(k_enc: &[u8; 32], info: &[u8], pt: &[u8]) -> Vec<u8> {
    let mut hasher = Shake256::default();
    hasher.update(k_enc);
    hasher.update(info);
    let mut xof = hasher.finalize_xof();
    let mut ks = vec![0u8; pt.len()];
    xof.read(&mut ks);
    pt.iter().zip(ks).map(|(p, k)| p ^ k).collect()
}

/// NSSA-style KDF: HKDF-SHA256(ss_bytes) with domain-separated salt+info.
/// Host-only (but compiles under no_std).
#[cfg(not(target_arch = "riscv32"))]
pub fn nssa_kdf(
    ss_bytes: [u8; 32],
    epk_bytes: &[u8],
    ipk_bytes: &[u8],
    commitment: [u8; 32],
    out_index: u32,
) -> ([u8; 32], Vec<u8>) {
    use hkdf::Hkdf;
    use sha2::{Sha256, Digest as _};

    // salt = SHA256("NSSA/v0.1/KDF-SHA256")
    let mut h = Sha256::new();
    // Disambiguate to avoid the "multiple `update`" error:
    sha2::Digest::update(&mut h, b"NSSA/v0.1/KDF-SHA256");
    let salt = h.finalize();

    // info = "NSSA/v0.1/enc" || Epk || Ipk || commitment || le(out_index)
    let mut info = Vec::new();
    info.extend_from_slice(b"NSSA/v0.1/enc");
    info.extend_from_slice(epk_bytes);
    info.extend_from_slice(ipk_bytes);
    info.extend_from_slice(&commitment);
    info.extend_from_slice(&out_index.to_le_bytes());

    // HKDF-Extract/Expand
    let hk = Hkdf::<Sha256>::new(Some(&salt), &ss_bytes);

    let mut k_enc = [0u8; 32];
    hk.expand(&info, &mut k_enc).expect("HKDF expand");

    (k_enc, info)
}
