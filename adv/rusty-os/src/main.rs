#![no_std] // disable standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // custom panic handler

    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // custom entry point

    println!("Namaste, world{}", "!");
    panic!("test panic message");

    loop {}
}
