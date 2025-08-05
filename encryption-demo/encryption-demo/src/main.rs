use encryption_demo::*;
use rand::Rng;

fn main() -> anyhow::Result<()> {
    let msg = b"zk-encrypted hello world!";
    let mut rng = rand::thread_rng();
    let key: [u8; 32] = rng.gen();
    let nonce: [u8; 12] = rng.gen();

    let (ciphertext, _proof) = encrypt_chacha20(&key, &nonce, msg)?;
    println!("ciphertext = {}", hex::encode(ciphertext));
    println!("proof    OK");
    Ok(())
}
