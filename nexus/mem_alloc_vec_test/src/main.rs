#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

extern crate alloc;

use alloc::vec::Vec;

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

pub fn vec_alloc(n: u32) -> u32 {

    let mut vvec = alloc_vec();

    push_vec(&mut vvec);

    pop_vec(&mut vvec);

    0
}

#[nexus_rt::main]
fn main() {
    let n = 7;
    let result = vec_alloc(n);
    assert_eq!(result, 13);
}