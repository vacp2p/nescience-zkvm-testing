#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

use ark_ec::{CurveGroup, PrimeGroup};

#[nexus_rt::main]
fn main() {
    let g1 = ark_bn254::G1Projective::generator();
    let g2 = g1 + g1;
    let g3 = g1 + g2;
    let g4 = g1 + g3;
    let g5 = g1 + g4;
        
    let s1 = ark_bn254::Fr::from(87329482u64);
    let s2 = ark_bn254::Fr::from(37264829u64);
    let s3 = ark_bn254::Fr::from(98098098u64);
    let s4 = ark_bn254::Fr::from(63980948u64);
    let s5 = ark_bn254::Fr::from(15098098u64);

    let _ = (g1*s1 + g2*s2 + g3*s3 + g4*s4 + g5*s5).into_affine();
}
