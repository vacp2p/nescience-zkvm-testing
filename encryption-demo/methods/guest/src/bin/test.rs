use risc0_zkvm::guest::env;

fn main() {
    let a: u128 = env::read();
    env::commit(&a);
}
