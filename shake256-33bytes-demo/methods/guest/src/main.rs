#![no_std]
#![no_main]
extern crate alloc;

use alloc::vec::Vec;
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

// ---------- module 1 ----------
mod ser_bytes33 {
    use core::fmt;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use serde::de::{self, SeqAccess, Visitor};

    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Bytes33(pub [u8; 33]);

    impl Serialize for Bytes33 {
        fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
            s.serialize_bytes(&self.0)
        }
    }
    impl<'de> Deserialize<'de> for Bytes33 {
        fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            struct V;
            impl<'de> Visitor<'de> for V {
                type Value = Bytes33;
                fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    write!(f, "exactly 33 bytes")
                }
                fn visit_bytes<E: de::Error>(self, v: &[u8]) -> Result<Self::Value, E> {
                    if v.len() != 33 { return Err(E::invalid_length(v.len(), &"33 bytes")); }
                    let mut a = [0u8; 33];
                    a.copy_from_slice(v);
                    Ok(Bytes33(a))
                }
                fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                    let mut a = [0u8; 33];
                    for i in 0..33 {
                        a[i] = seq.next_element()?
                            .ok_or_else(|| de::Error::invalid_length(i, &"33 bytes"))?;
                    }
                    Ok(Bytes33(a))
                }
            }
            d.deserialize_bytes(V)
        }
    }
    impl AsRef<[u8; 33]> for Bytes33 { fn as_ref(&self) -> &[u8; 33] { &self.0 } }
    impl AsRef<[u8]> for Bytes33 { fn as_ref(&self) -> &[u8] { &self.0 } }
}
use ser_bytes33::Bytes33; // bring into crate scope
// ---------- end module 1 ----------

// ---------- module 2 ----------
mod crypto {
    #![allow(clippy::needless_borrows_for_generic_args)]

    extern crate alloc;
    use alloc::vec; 
    use alloc::vec::Vec;
    use sha2::{Digest, Sha256};
    use tiny_keccak::{Hasher, Shake};

    pub fn nssa_kdf(
        ss_bytes: [u8; 32],
        epk: &[u8; 33],
        ipk: &[u8; 33],
        commitment: &[u8; 32],
        out_index: u32,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update( b"NSSA/v0.1/KDF-SHA256");
        sha2::Digest::update(&mut hasher, &ss_bytes);
        sha2::Digest::update(&mut hasher, &epk[..]);
        sha2::Digest::update(&mut hasher, &ipk[..]);
        sha2::Digest::update(&mut hasher, &commitment[..]);
        sha2::Digest::update(&mut hasher, &out_index.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn enc_xor_shake256(key: &[u8; 32], info: &[u8], pt: &[u8]) -> Vec<u8> {
        let mut sh = Shake::v256();
        tiny_keccak::Hasher::update(&mut sh, b"NSSA/v0.1/shake-ks");
        tiny_keccak::Hasher::update(&mut sh, &key[..]);
        tiny_keccak::Hasher::update(&mut sh, info);

        let mut ks = vec![0u8; pt.len()];
        sh.finalize(&mut ks);

        let mut ct = vec![0u8; pt.len()];
        for (i, &b) in pt.iter().enumerate() {
            ct[i] = b ^ ks[i];
        }
        ct
    }
}
use crypto::{enc_xor_shake256, nssa_kdf};
// ---------- end module 2 ----------

// ---------- plain types at crate root ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncInput {
    pub ss_bytes: [u8; 32],
    pub epk_bytes: Bytes33,
    pub ipk_bytes: Bytes33,
    pub commitment: [u8; 32],
    pub out_index: u32,
}
// ---------- end types ----------

// ---------- entrypoint at crate root  ----------
risc0_zkvm::guest::entry!(guest_main);

pub fn guest_main() {
    let EncInput { ss_bytes, epk_bytes, ipk_bytes, commitment, out_index } = env::read();

    let mut info: Vec<u8> = Vec::new();
    info.extend_from_slice(epk_bytes.as_ref());
    info.extend_from_slice(ipk_bytes.as_ref());
    info.extend_from_slice(&commitment);

    let k_enc = nssa_kdf(ss_bytes, epk_bytes.as_ref(), ipk_bytes.as_ref(), &commitment, out_index);

    let pt: &[u8] = b"hello";
    let ct = enc_xor_shake256(&k_enc, &info, pt);

    env::commit_slice(&ct);
}
// ---------- end entry ----------
