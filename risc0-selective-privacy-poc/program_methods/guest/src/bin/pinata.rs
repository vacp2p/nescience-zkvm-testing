use core::{account::Account, hash};
use risc0_zkvm::guest::env;

const TARGET_HASH: [u32; 8] = [
    1363824975, 720119575, 717909014, 2043925380, 717793160, 1495780600, 1253022833, 116132328,
];
const PINATA_ACCOUNT_ADDR: [u32; 8] = [0xcafe; 8];
const PINATA_PRIZE: u128 = 100;

/// A Piñata program
/// To be used both in public and privacy contexts.
fn main() {
    // Read input accounts. It is expected to receive only two accounts: [pinata_account, winner_account]
    let mut input_accounts: Vec<Account> = env::read();

    // Read claimed preimage
    let preimage: Vec<u32> = env::read();

    // Unpack accounts.
    assert_eq!(input_accounts.len(), 2);
    let [winner_account] = input_accounts.split_off(1).try_into().unwrap();
    let [pinata_account] = input_accounts.try_into().unwrap();

    // Check that the given `pinata_account` is correct
    assert_eq!(pinata_account.address, PINATA_ACCOUNT_ADDR);

    // Check that the piñata account has enough balance to pay the prize
    assert!(pinata_account.balance >= PINATA_PRIZE);

    // Check that the preimage is correct.
    assert_eq!(hash(&preimage), TARGET_HASH);

    // Pay the prize
    let mut winner_account_post = winner_account.clone();
    let mut pinata_account_post = pinata_account.clone();
    pinata_account_post.balance -= PINATA_PRIZE;
    winner_account_post.balance += PINATA_PRIZE;

    env::commit(&vec![
        pinata_account,
        winner_account,
        pinata_account_post,
        winner_account_post,
    ]);
}
