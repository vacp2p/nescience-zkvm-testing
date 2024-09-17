use alloy_sol_types::sol;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint32 n;
        uint32 a;
        uint32 b;
    }
}

pub const CAP_LIM: usize = 10000;
pub const TO_INSERT_ITEM: usize = 145;

macro_rules! write_tests_vec {
    ($t:ty, $fn_name_1:ident, $fn_name_2:ident, $fn_name_3:ident, $fn_name_4:ident) => {
        pub fn $fn_name_1() -> $t {
            <$t>::with_capacity(CAP_LIM)
        }

        pub fn $fn_name_2(vecc: &mut $t) {
            for _ in 1..CAP_LIM {
                vecc.push(TO_INSERT_ITEM);
            }
        }

        pub fn $fn_name_3(vecc: &mut $t) {
            for _ in 0..(CAP_LIM + 1) {
                vecc.push(TO_INSERT_ITEM);
            }
        }

        pub fn $fn_name_4(vecc: &mut $t) {
            for _ in 1..CAP_LIM {
                vecc.pop();
            }
        }
    };
}

write_tests_vec!(Vec<usize>, alloc_vec, push_vec, dyn_alloc_vec, pop_vec);

pub fn vec_alloc(n: u32) -> (u32, u32) {

    let mut vecc = alloc_vec();

    push_vec(&mut vecc);

    pop_vec(&mut vecc);
    
    (0, 0)
}