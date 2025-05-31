#![allow(bad_asm_style)]
#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c:i32, n:usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        *s.add(i) = c;
    }
    s
}

#[no_mangle]
pub extern "C" fn kernel_main() -> !{
    let mut terminal_writer = TerminalWriter::new();
    terminal_writer.write(b"We did it, a rust kernel!");
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}