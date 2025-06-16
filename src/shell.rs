#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;
use core::str;

// MY OWN C BINDINGS AND STDLIB
mod stdlib;
use stdlib::{print, uprint, getch, pwd, matchpcchar, splitft, cd, quit, exec, input, CChar, PCChar};

#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {    
    print("Panic! ".as_ptr() as PCChar);
    if let Some(location) = info.location() {

	print("PANICKED".as_ptr() as PCChar);

        let file = location.file();
        let line = location.line();
        let column = location.column();
	print("FILE:    ".as_ptr() as PCChar);
        print(file.as_ptr() as PCChar);
	print("LINE:     ".as_ptr() as PCChar);
        uprint(line);
	print("COLUMN:    ".as_ptr() as PCChar);
        uprint(column);
    }
    quit(69)
}

static mut HISTORY_BUFFER: [[u8; 256]; 16] = [[0; 256]; 16];
static mut HISTORY_STRING: [PCChar; 16] = [core::ptr::null(); 16]; 
static mut HISTORY_INDEX : usize = 0;

pub unsafe fn history(s: PCChar) {
    use core::ptr;

    if HISTORY_INDEX >= HISTORY_STRING.len() {
        HISTORY_INDEX = 0;
    }
    let mut len = 0;
    while *s.add(len) != 0 && len < 255 {
        len += 1;
    }
    ptr::copy_nonoverlapping(s as *const u8, HISTORY_BUFFER[HISTORY_INDEX].as_mut_ptr(), len);
    HISTORY_BUFFER[HISTORY_INDEX][len] = 0;

    HISTORY_STRING[HISTORY_INDEX] = HISTORY_BUFFER[HISTORY_INDEX].as_ptr() as PCChar;
    HISTORY_INDEX += 1;
}

pub unsafe fn initialize_terminal() {
    let path = pwd();
    print(path); 
    print(" -> ".as_ptr() as PCChar);
    let buf = input();
    let cmd = buf.as_ptr() as *const CChar;
    static EXIT: &[u8] = b"exit\n\0";
    static CD: &[u8] = b"cd\0";
    static HISTORY_BACK: &[u8] = b"back\0";
    
    let token = splitft(cmd as *const u8);
    
    if matchpcchar(token.word, EXIT.as_ptr()) {
	quit(0);
    } else if matchpcchar(token.word, CD.as_ptr()) {
 
	static mut BUFFER: [u8; 128] = [0; 128];
 	let mut len = 0;
	let mut p = token.rest;
	for i in 0..BUFFER.len() {
            BUFFER[i] = 0;
	}
	loop {
            let c = *p;
            if c == 0 || c == b'\n' || c == b'\r' || len >= BUFFER.len() - 1 {
		break;
            }
            BUFFER[len] = c;
            len += 1;
            p = p.add(1);
	}
	BUFFER[len] = 0;
	
	cd(BUFFER.as_ptr());

    } else {
	exec(token.word as *const CChar);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> i32 {
    loop {
        initialize_terminal();
    }
}

#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

