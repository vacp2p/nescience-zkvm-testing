// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::vec::Vec;

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

use nalgebra::Matrix2;
use risc0_zkvm::guest::env;

fn main() {
    let iterations: u32 = env::read();
    let answer = vec_alloc(iterations);
    env::commit(&answer);
}

fn vec_alloc(n: u32) -> u64 {
    let mut vvec = alloc_vec();

    push_vec(&mut vvec);

    pop_vec(&mut vvec);

    0
}
