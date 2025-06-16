#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;
use core::str;

// MY OWN C BINDINGS AND STDLIB
mod stdlib;
use stdlib::{print, uprint, pu_to_ar, cstr_to_str, cd, quit, exec, input, CChar, PCChar};

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

static mut HISTORY_STRING: [PCChar; 16] = [core::ptr::null(); 16]; 
static mut HISTORY_INDEX : usize = 0;

pub unsafe fn history(s: PCChar) {
    if HISTORY_INDEX >= HISTORY_STRING.len() {
        HISTORY_INDEX = 0;
    }
    HISTORY_STRING[HISTORY_INDEX] = s;
    HISTORY_INDEX += 1;
}

pub unsafe fn initialize_terminal() {
    let pathter = exec(b"pwd\0".as_ptr() as PCChar);
    print(pathter.output as PCChar); 
    print("-> ".as_ptr() as PCChar);
    let buf = input();
    let cmd = buf.as_ptr() as *const CChar;
    let str = cstr_to_str(cmd);
    let mut str_split = str.split(" "); 
    match str_split.next().unwrap_or("") {
	"exit\n" => {
	    quit(0);
	}
	"cd" => {
	    static mut BUFFER: [u8; 128] = [0; 128];
	    
	    let pathlucky = str_split.next().unwrap_or("");
	    let trimmed = pathlucky.trim_end_matches(|c| c == '\n' || c == '\r');
	    let bytes = trimmed.as_bytes();
	    let len = bytes.len().min(BUFFER.len() - 1);
	    BUFFER[..len].copy_from_slice(&bytes[..len]);
	    BUFFER[len] = 0;
	    
	    cd(BUFFER.as_ptr());
	}
	_ => {
	    history(cmd);
	    exec(cmd);
	}
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
