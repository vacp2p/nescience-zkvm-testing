use risc0_zkvm::guest::env;
use pasta_curves::group::Group;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    let g1 = pasta_curves::vesta::Point::generator();
    let g2 = g1 + g1;
    let g3 = g1 + g2;
    let g4 = g1 + g3;
    let g5 = g1 + g4;
        
    let s1 = pasta_curves::vesta::Scalar::from(87329482u64);
    let s2 = pasta_curves::vesta::Scalar::from(37264829u64);
    let s3 = pasta_curves::vesta::Scalar::from(98098098u64);
    let s4 = pasta_curves::vesta::Scalar::from(63980948u64);
    let s5 = pasta_curves::vesta::Scalar::from(15098098u64);

    let _ = g1*s1 + g2*s2 + g3*s3 + g4*s4 + g5*s5;
    
    // write public output to the journal
    env::commit(&input);
}
