
#[cfg(not(rust_analyzer))]
pub mod methods {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}

#[cfg(not(rust_analyzer))]

#[cfg(rust_analyzer)]
pub const GUEST_ELF: &[u8] = &[];
#[cfg(rust_analyzer)]
pub const GUEST_ID: [u32; 8] = [0; 8];



use hkdf::Hkdf;
use sha2::{Digest as Sha2Digest, Sha256};
use sha3::{Shake256, digest::{Update, ExtendableOutput, XofReader}};
use serde::{Deserialize, Serialize};
//use serde_big_array::BigArray;


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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KdfInputs {
    pub epk_bytes: Vec<u8>,
    pub ipk_bytes: Vec<u8>,
    pub commitment: [u8; 32],
    pub out_index: u32,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct EncInput {
    pub plaintext: Vec<u8>,
    pub ss_bytes: [u8; 32],     // ECDH x-coordinate (sender view)
    pub epk_bytes: Vec<u8>,
    pub ipk_bytes: Vec<u8>,
    pub commitment: [u8; 32],
    pub out_index: u32,
}

/// KDF per NSSA spec: HKDF-SHA256 with fixed salt and domain-separated info.
pub fn nssa_kdf(
    ss_bytes: &[u8; 32],
    epk: &[u8],
    ipk: &[u8],
    commitment: &[u8; 32],
    out_index: u32,
) -> ([u8; 32], Vec<u8>) {
    // salt = SHA256("NSSA/v0.1/KDF-SHA256")
    let mut hasher = Sha256::new();
    Sha2Digest::update(&mut hasher, b"NSSA/v0.1/KDF-SHA256"); // avoids E0034
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
    hk.expand(&info, &mut k_enc)
        .expect("HKDF-Expand must produce 32 bytes");

    (k_enc, info)
}

/// SHAKE256-XOF keystream XOR (symmetric).
pub fn enc_xor_shake256(k_enc: &[u8; 32], ad: &[u8], pt: &[u8]) -> Vec<u8> {
    let mut shake = Shake256::default();
    shake.update(b"NSSA/v0.1/ENC/SHAKE256"); // domain sep
    shake.update(k_enc);
    shake.update(ad);
    let mut xof = shake.finalize_xof();

    let mut ks = vec![0u8; pt.len()];
    xof.read(&mut ks);

    pt.iter().zip(ks).map(|(p, k)| p ^ k).collect()
}
