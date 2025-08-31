use risc0_zkvm::guest::env;
use group::Group;

fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: u32 = env::read();

    // TODO: do something with the input
    let g1 = jubjub::SubgroupPoint::generator();
    let g2 = g1 + g1;
    let g3 = g1 + g2;
    let g4 = g1 + g3;
    let g5 = g1 + g4;
        
    let s1 = jubjub::Fr::from(87329482u64);
    let s2 = jubjub::Fr::from(37264829u64);
    let s3 = jubjub::Fr::from(98098098u64);
    let s4 = jubjub::Fr::from(63980948u64);
    let s5 = jubjub::Fr::from(15098098u64);

    let _ = g1*s1 + g2*s2 + g3*s3 + g4*s4 + g5*s5;

    // write public output to the journal
    env::commit(&input);
}
