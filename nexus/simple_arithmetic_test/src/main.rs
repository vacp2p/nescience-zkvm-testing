#![cfg_attr(target_arch = "riscv32", no_std, no_main)]

pub fn hept(n: u32) -> u32 {

    for i in 0..100 {
        let hept = (5*i*i - 3*i)/2;
    }
    0
}

#[nexus_rt::main]
fn main() {
    let n = 7;
    let result = hept(n);
}