#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

//continue from https://os.phil-opp.com/freestanding-rust-binary/#disabling-unwinding
