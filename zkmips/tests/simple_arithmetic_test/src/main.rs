#![no_std]
#![no_main]

zkm_runtime::entrypoint!(main);

pub fn main() {
    for i in 0..100 {
        let hept = (5*i*i - 3*i)/2;
    }
}