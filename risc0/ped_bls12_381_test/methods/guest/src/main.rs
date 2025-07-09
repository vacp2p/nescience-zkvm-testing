use risc0_zkvm::guest::env;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let g1 = bls12_381::G1Affine::generator();
    let g1_proj = bls12_381::G1Projective::from(g1);
    let g2 = g1_proj + g1;
    let g3 = g1 + g2;
    let g4 = g1 + g3;
    let g5 = g1 + g4;

    let s1 = bls12_381::Scalar::from(87329482u64);
    let s2 = bls12_381::Scalar::from(37264829u64);
    let s3 = bls12_381::Scalar::from(98098098u64);
    let s4 = bls12_381::Scalar::from(63980948u64);
    let s5 = bls12_381::Scalar::from(15098098u64);

    let _ = s1*g1 + s2*g2 + s3*g3 + s4*g4 + s5*g5;
 
   // write public output to the journal
    env::commit(&input);
}
