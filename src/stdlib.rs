#![no_std]

use core::{slice};

pub type CChar = i8;
pub type PCChar = *const i8;
pub type PCharA<const N: usize> = [*const i8; N];
pub const STDOUT : i32 = 1;
pub const STDIN : i32 = 0;

#[derive(Clone, Copy)]
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
    fn getcwd(buf: *mut i8, size: usize) -> PCChar;
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

pub unsafe fn pwd() -> PCChar {
    static mut BUF: [CChar; 256] = [0; 256];
    let ptr = getcwd(BUF.as_mut_ptr(), BUF.len());
    if ptr.is_null() {
        return b"<unknown>\0".as_ptr() as PCChar;
    }
    BUF.as_ptr()
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

pub unsafe fn getch() -> u8 {
    let mut buf: u8 = 0;
    let n = read(0 as usize, &mut buf as *mut u8, 1);
    if n <= 0 {
        0
    } else {
        buf
    }
}


pub unsafe fn exec(command: PCChar) -> i32 {
    system(command)
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

#[derive(Clone, Copy)]
pub struct Token {
    pub word: *const u8,
    pub rest: *const u8,
}

pub unsafe fn splitft(mut input: *const u8) -> Token {
    static mut WORD: [u8; 64] = [0; 64];
    let mut i = 0;

    while *input != 0 && *input != b' ' && i < 63 {
        WORD[i] = *input;
        input = input.add(1);
        i += 1;
    }
    WORD[i] = 0;

    if *input == b' ' {
        input = input.add(1);
    }

    Token {
        word: WORD.as_ptr(),
        rest: input,
    }
}

pub unsafe fn matchpcchar(a: *const u8, b: *const u8) -> bool {
    let mut i = 0;
    loop {
        let ca = *a.add(i);
        let cb = *b.add(i);
        if ca != cb {
            return false;
        }
        if ca == 0 {
            return true;
        }
        i += 1;
    }
}
