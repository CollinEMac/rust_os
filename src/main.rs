// https://os.phil-opp.com/freestanding-rust-binary/

// Do not link to the standard library
#![no_std]
#![no_main]
// #![feature(asm)]

mod vga_buffer;

use core::panic::PanicInfo;

// Implement a panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World {}", "!");
    // panic!("Some panic message");

    loop {}
}
