#![no_std] // disable standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // custom panic handler
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // custom entry point
    loop {}
}

// ctd from https://os.phil-opp.com/minimal-rust-kernel/#target-specification
