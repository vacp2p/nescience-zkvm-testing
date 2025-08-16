pub mod methods {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}


use serde::{Deserialize, Serialize};
// ---------- 33-byte wrapper (public) ----------
pub mod ser_bytes33 {                // (public so main.rs can use it)
    use core::fmt;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

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
            impl<'de> de::Visitor<'de> for V {
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
            }
            d.deserialize_bytes(V)
        }
    }
    impl AsRef<[u8; 33]> for Bytes33 { fn as_ref(&self) -> &[u8; 33] { &self.0 } }
    impl AsRef<[u8]> for Bytes33 { fn as_ref(&self) -> &[u8] { &self.0 } }

    // this is an optional QoL: it allows `epk_bytes.into()` from a plain array
    impl From<[u8; 33]> for Bytes33 {                           
        fn from(a: [u8; 33]) -> Self { Bytes33(a) }
    }
}
pub use ser_bytes33::Bytes33; //  re-export
// --------------------------------------------------

// ---------- crypto (public) ----------
pub mod crypto {                         // makes the module public
    use sha2::{Digest, Sha256};
    use tiny_keccak::{Hasher, Shake};

    pub fn nssa_kdf(                     // keeps public
        ss_bytes: [u8; 32],
        epk: &[u8; 33],
        ipk: &[u8; 33],
        commitment: &[u8; 32],
        out_index: u32,
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"NSSA/v0.1/KDF-SHA256");
        hasher.update(&ss_bytes);
        hasher.update(&epk[..]);
        hasher.update(&ipk[..]);
        hasher.update(&commitment[..]);
        hasher.update(&out_index.to_le_bytes());
        hasher.finalize().into()
    }

    pub fn enc_xor_shake256(key: &[u8; 32], info: &[u8], pt: &[u8]) -> Vec<u8> {
        let mut sh = Shake::v256();
        sh.update(b"NSSA/v0.1/shake-ks");
        sh.update(&key[..]);
        sh.update(info);

        let mut ks = vec![0u8; pt.len()];
        sh.finalize(&mut ks);

        let mut ct = vec![0u8; pt.len()];
        for (i, &b) in pt.iter().enumerate() {
            ct[i] = b ^ ks[i];
        }
        ct
    }
}

// Re-export so callers can `use shake256_demo::{enc_xor_shake256, nssa_kdf};`
pub use crypto::{enc_xor_shake256, nssa_kdf};                    
// ------------------------------------------------------------

// ---------- types ----------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncInput {
    pub ss_bytes: [u8; 32],
    pub epk_bytes: Bytes33,
    pub ipk_bytes: Bytes33,
    pub commitment: [u8; 32],
    pub out_index: u32,
}

// ------------------------------------------------------------

pub fn build_info(epk: &Bytes33, ipk: &Bytes33) -> Vec<u8> {
    let mut info = Vec::with_capacity(66);
    info.extend_from_slice(epk.as_ref());
    info.extend_from_slice(ipk.as_ref());
    info
}

pub fn example_use(input: &EncInput) -> ([u8; 32], Vec<u8>) {
    let info = build_info(&input.epk_bytes, &input.ipk_bytes);
    let k: [u8; 32] = [0u8; 32];
    (k, info)
}
