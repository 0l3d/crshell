#![no_std]

use core::{slice};

pub type CChar = i8;
pub type PCChar = *const i8;
pub type PCharA<const N: usize> = [*const i8; N];
pub const STDOUT : i32 = 1;
pub const STDIN : i32 = 0;

pub struct ExecResult {
    pub output: PCChar,
    pub status: i32,
}

extern "C" {
    fn write(fd: usize, buf: *const u8, count: usize) -> isize;
    fn exit(code: i32) -> !;
    fn system(command: PCChar) -> i32;
    fn read(fd: usize, buf: *mut u8, count: usize) -> isize;
    fn chdir(path: PCChar) -> i32;
}

pub unsafe fn print(s: PCChar) {
    if s.is_null() {
        return;
    }
    let mut len = 0;
    while *s.add(len) != 0 {
        len += 1;
    }
    write(1, s as *const u8, len);
}

pub unsafe fn pu_to_ar(mut array: [PCChar; 16], s: PCChar, index: usize) {
    if index >= array.len() {
        print(b"Index out of bounds.\0".as_ptr() as PCChar);
        panic!("Index err");
    }
    array[index] = s;
}

pub unsafe fn uprint(mut n: u32) {
    let mut buf = [0u8; 10];
    let mut i = 10;
    if n == 0 {
        let zero = b"0";
        write(1, zero.as_ptr(), 1);
        return;
    }
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let len = 10 - i;
    write(1, buf[i..].as_ptr(), len);
}

pub unsafe fn quit(code: i32) -> ! {
    exit(code);
}

pub unsafe fn exec(command: PCChar) -> ExecResult {
    static mut OUTPUT: [u8; 128] = [0; 128];

    let status = system(command);

    ExecResult {
        output: OUTPUT.as_ptr() as PCChar,
        status,
    }
}

pub unsafe fn input() -> [u8; 256] {
    let mut buffer = [0u8; 256];
    let _buf : isize = read(0, buffer.as_mut_ptr(), buffer.len() - 1);
    buffer	
}

pub unsafe fn ensure_null_terminated<'a>(input: *const u8, input_len: usize, output: &'a mut [u8]) -> Result<PCChar, ()> {
    if input_len + 1 > output.len() {
        return Err(());
    }
    for i in 0..input_len {
        output[i] = *input.add(i);
    }
    if input_len == 0 || *input.add(input_len - 1) != 0 {
        output[input_len] = 0;
        Ok(output.as_ptr() as PCChar)
    } else {
        Ok(output.as_ptr() as PCChar)
    }
}

pub unsafe fn cd(path: *const u8) -> i32 {
    if path.is_null() {
        return -1;
    }

    let mut len = 0;
    while *path.add(len) != 0 {
        len += 1;
        if len >= 255 {
            break;
        }
    }

    let mut buffer = [0u8; 256];
    let c_path = match ensure_null_terminated(path, len, &mut buffer) {
        Ok(cstr) => cstr,
        Err(_) => return -1,
    };

    chdir(c_path)
}

pub unsafe fn cstr_to_str(ptr: PCChar) -> &'static str {
    if ptr.is_null() {
        return "";
    }
    let mut len = 0;
    while *ptr.add(len) != 0 {
        len += 1;
    }
    let bytes = slice::from_raw_parts(ptr as *const u8, len);
    core::str::from_utf8(bytes).unwrap_or("")
}


