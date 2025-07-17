use core::{account::Account, hash};
use risc0_zkvm::guest::env;

// preimage is b"NSSA Selective privacy is great!"
const TARGET_HASH: [u32; 8] = [
    1363824975, 720119575, 717909014, 2043925380, 717793160, 1495780600, 1253022833, 116132328,
];
const PINATA_ACCOUNT_ADDR: [u32; 8] = [0xcafe; 8];
const PINATA_PRICE: u128 = 100;

/// A Piñata program
/// To be used both in public and private contexts.
fn main() {
    let mut input_accounts: Vec<Account> = env::read();
    let preimage: Vec<u32> = env::read();

    assert_eq!(input_accounts.len(), 2);
    let [winner_account] = input_accounts.split_off(1).try_into().unwrap();
    let [pinata_account] = input_accounts.try_into().unwrap();

    assert_eq!(pinata_account.address, PINATA_ACCOUNT_ADDR);
    assert!(pinata_account.balance >= PINATA_PRICE);
    assert_eq!(hash(&preimage), TARGET_HASH);

    let mut winner_account_post = winner_account.clone();
    let mut pinata_account_post = pinata_account.clone();
    pinata_account_post.balance -= PINATA_PRICE;
    winner_account_post.balance += PINATA_PRICE;

    env::commit(&vec![
        pinata_account,
        winner_account,
        pinata_account_post,
        winner_account_post,
    ]);
}
