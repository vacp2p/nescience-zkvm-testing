use anyhow::Result;
use risc0_zkvm::{default_prover, ExecutorEnv, Digest};

#[cfg(not(rust_analyzer))]
include!(concat!(env!("OUT_DIR"), "/methods.rs"));

#[test]
fn verify_rejects_wrong_image() -> Result<()> {
    let key = [0x42u8; 32];
    let nonce = [0x24u8; 12];
    let plaintext = b"bad id test".to_vec();

    let env = ExecutorEnv::builder()
        .write(&key)?.write(&nonce)?.write(&plaintext)?.build()?;

    let info = default_prover().prove(env, GUEST_ELF)?;

    // Intentionally bogus image id
    let bogus = Digest::from([0u32; 8]);
    assert!(info.receipt.verify(bogus).is_err(), "verification should fail");
    Ok(())
}
