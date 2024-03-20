#![no_std] // disable standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // custom panic handler
    loop {}
}

static HELLO: &[u8] = b"Namaste World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // custom entry point
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0x6;
        }
    }
    loop {}
}
