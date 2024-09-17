use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

pub fn hept(n: u32) -> (u32, u32) {

    for i in 0..100 {
        let hept = (5*i*i - 3*i)/2;
    }

    (0, 0)
}