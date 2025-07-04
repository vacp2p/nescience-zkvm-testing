#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use curve25519_dalek::constants;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;

#[nexus_rt::main]
fn main() {
    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let s1 = Scalar::from(87329482u64);
    let s2 = Scalar::from(37264829u64);
    let s3 = Scalar::from(98098098u64);
    let s4 = Scalar::from(63980948u64);
    let s5 = Scalar::from(15098098u64);

    let g1 = constants::RISTRETTO_BASEPOINT_POINT;
    let g2 = g1 + g1;
    let g3 = g1 + g2;
    let g4 = g1 + g3;
    let g5 = g1 + g4;

    let nums = [s1,s2,s3,s4,s5];
        
    let _ = RistrettoPoint::multiscalar_mul(&nums, &[g1,g2,g3,g4,g5]);
}
