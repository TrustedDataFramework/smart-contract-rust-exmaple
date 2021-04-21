#![no_std]
#[macro_use]
extern crate alloc;
extern crate core;

use alloc::{vec::Vec};
use alloc::string::*;
use alloc::boxed::Box;
use core::mem;

#[macro_export]
macro_rules! use_wee_alloc {
    () => {                
        // Use `wee_alloc` as the global allocator.
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
        
        #[macro_use]
        extern crate alloc;
        extern crate core;  
        
        #[panic_handler]
        fn panic(info: &core::panic::PanicInfo) -> ! {
            log(&format!("{:?}", info));
        
            loop {}
        }        
    };
}

// 本质上 Box<String> 和 &String 是一样的
extern "C" {
    pub fn _log(a: u64); 
    pub fn _context(t: u64, a: u64) -> u64;
}

pub fn log(s: &str) {
    unsafe {
        _log(forget(s.to_string()) as u64)
    }
}

#[no_mangle]
pub fn __malloc(size: u64) -> u64 {
    let bytes = vec![0u8; size as usize];
    forget_bytes(bytes)
}

// convert t to raw, 
fn forget_bytes(mut t: Vec<u8>) -> u64 {
    let raw = t.as_mut_ptr();
    let ret = (raw as usize) as u64;
    mem::forget(t);
    ret
}


fn remember_bytes(ptr: u64, size: u64) -> Vec<u8> {
    unsafe {
        let raw = ptr as *mut u8;
        Vec::from_raw_parts(raw, size as usize, size as usize)
    }
}

fn remember<T>(p: u64) -> T {
    unsafe {
        let b = Box::from_raw(p as *mut T);
        return *b;
    }
}

fn forget<T>(d: T) -> *mut T {
    let r = Box::new(d);
    Box::leak(r)
}

pub fn ret<T>(d: T) -> &'static T {
    let r = Box::new(d);
    Box::leak(r)
}


#[no_mangle]
pub unsafe fn __change_t(t: u64, ptr: u64, size: u64) -> u64 {
    let v = remember_bytes(ptr, size);
    // string
    if t == 4 {
        let s = String::from_utf8_unchecked(v);
        return forget(s) as u64;
    }

    if t == 5 {
        return forget(v) as u64;
    }

    return 0;
}


#[no_mangle]
pub fn __peek(ptr: u64, t: u64) -> u64 {
    // convert string to utf8 bytes
    if t == 4 {
        let p: String = remember(ptr);
        let bytes: Vec<u8> = p.into_bytes();
        let bytes_len = bytes.len();
        return ((forget_bytes(bytes)) << 32) | (bytes_len as u64);
    }

    if t == 5 {
        let p: Vec<u8> = remember(ptr);
        let bytes_len = p.len();
        return ((forget_bytes(p)) << 32) | (bytes_len as u64);
    }    
    return 0;
}
