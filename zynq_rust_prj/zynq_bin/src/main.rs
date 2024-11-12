#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;

pub mod uart;

extern "C" {
    static _stack_start: u32;
    static _stack_end: u32;
}

pub fn delay() {
    for _ in 0..1_000_000 {
        unsafe {
            core::ptr::read_volatile(&0);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Set the stack pointer to the end of the stack
        asm!(
            "ldr sp, =_stack_end",
            options(nomem, nostack, preserves_flags)
        );
    }

    // Call main
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
