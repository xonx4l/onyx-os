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

#[allow(dead_code)]
enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    LightMagenta =13,
    LightBrown = 14,
    White = 15,
}

fn vga_entry_color(fg: VgaColor, bg: VgaColor) -> u8 {
    fg as u8 | (bg as u8) << 4 
}

fn vga_entry(uc: u8, color:u8) -> u16 {
    uc as u16 | (color as u16) << 8 
}

const VGA_HEIGHT: usize = 80;
const VGA_WIDTH: usize = 25;

struct TerminalWriter [
    terminal_row: usize ,
    terminal_column: usize,
    terminal_color: u8,
    terminal_buffer: *mut u16,
]

impl TerminalWriter {
    fn new() -> TerminalWriter {
        let terminal_row  = 0;
        let terminal_column = 0;
        let terminal_color =
            vga_entry_color(VgaColor::LightGrey, VgaColor::Black);
        let terminal_buffer = 0xB8000 as *mut u16;
    }
}