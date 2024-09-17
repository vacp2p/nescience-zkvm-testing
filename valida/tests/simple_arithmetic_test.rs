#![no_std]
#![feature(start)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    let i = 11;
    let hept = (5*i*i + 3*i)/2;
    if hept == 286 {
        return 1;
    }
    else {
        return 0;
    }
}


