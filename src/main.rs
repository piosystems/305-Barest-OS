#![no_std]
#![no_main]
//#![feature(core_intrinsics)] //feature here because is outside standard lib

mod cursor;
use cursor::*;
use core::str;

//use core::intrinsics;
use x86_64::instructions::hlt;

//static HELLO: &[u8] = b"Hello World! Just written my first kernel without host OS";

//Add struct for colours
#[allow(dead_code)] //do not complain if a declaration is unused
#[derive(Debug, Clone, Copy)]
#[repr(u8)] //force one byte alignment in memory
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

fn write_position(framebuffer: *mut u8, row: usize, column: usize, str: &str, foreground: Colour, background: Colour) {
    if row > BUFFER_HEIGHT - 1 {
        //TODO: implement scroll
        panic!("Row should be from 0 to 24")
    }

    let initial_position = (row * BUFFER_WIDTH) + column;
    
    let colour = || -> u8 {
        let fg = foreground as u8;
        let bg = (background as u8) << 4;
        //let blink = 0b00000001 as u8; // Set least significant bit as flag for blink. Not blinking
        //let underline = 0b10000000 as u8; //set most significant bit for underline. Not working
        //underline| bg | fg | blink //combine
        bg | fg //combine
    };

    
    for (i, &byte) in str.as_bytes().iter().enumerate() {
        
        let count = i + initial_position;
        unsafe {
            framebuffer.offset(count as isize * 2).write_volatile(byte);
            framebuffer.offset(count as isize * 2 + 1).write_volatile(colour() );
            //same effect as above
            //*framebuffer.offset(count as isize * 2) = byte;
            //*framebuffer.offset(count as isize * 2 + 1) = colour();
        }
    }
    
    //reposition the cursor at the end of the last char written
    let mut new_column_position = (column + str.len()) as u16;
    let mut new_row_position = row as u16;
    if new_column_position + 1 > 80 {
        new_column_position = 0;//start from first column position
        new_row_position+=1; //advance row to the next line
    }
    if new_row_position > 25 {
        //should scroll here. For now, just leave it at the last line
        new_row_position = 25
    }

    update_cursor(new_column_position, new_row_position);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let framebuffer = 0xb8000 as *mut u8; //The VGA text buffer is accessible via memory-mapped I/O to the address 0xb8000

    /*for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *framebuffer.offset(i as isize * 2) = byte;
            *framebuffer.offset(i as isize * 2 + 1) = 0xb; //write with light cyan colour. You can change this
        }
    }*/

    write_position(framebuffer, 1, 3, "Test string", Colour::LightBlue, Colour::Black);

    write_position(framebuffer, 20, 3, "Test string again", Colour::LightGreen, Colour::Black);

    write_position(
        framebuffer,
        30,
        7,
        "Test string again and again",
        Colour::Cyan,
        Colour::Brown
    );

    loop{
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    //intrinsics::abort();
    loop{
        hlt();
    }
}
