use anyhow::Result;
use hex::ToHex;
use risc0_zkvm::{default_prover, ExecutorEnv};
use shake256_demo::{EncInput, enc_xor_shake256, nssa_kdf, methods};

fn main() -> Result<()> {
    // Example inputs.
    let ss_bytes   = [7u8; 32];
    let epk_bytes  = vec![1u8; 33];
    let ipk_bytes  = vec![2u8; 33];
    let commitment = [3u8; 32];
    let out_index  = 0u32;
    let plaintext  = b"hello NSSA!".to_vec();

    // Derive (k_enc, info) once on the host
    let (k_enc, info) = nssa_kdf(ss_bytes, &epk_bytes, &ipk_bytes, commitment, out_index);

    // Build guest input
    let input = EncInput { plaintext: plaintext.clone(), k_enc, info: info.clone() };

    let env = ExecutorEnv::builder()
        .write(&input)?
        .build()?;

    // Prove
    let receipt = default_prover().prove(env, methods::GUEST_ELF)?.receipt;

    // Get guest ciphertext
    let guest_ct: Vec<u8> = receipt.journal.decode()?;

    // Compute host ciphertext the same way
    let host_ct = enc_xor_shake256(&k_enc, &info, &plaintext);

    println!("guest ct: {}", guest_ct.encode_hex::<String>());
    println!("host  ct: {}", host_ct.encode_hex::<String>());

    assert_eq!(guest_ct, host_ct);
    Ok(())
}
