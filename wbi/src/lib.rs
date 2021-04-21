#![no_std]
#[macro_use]
extern crate alloc;
extern crate core;
extern crate rlp;

mod u256;
mod address;
pub mod db;
pub mod context;

use alloc::{vec::Vec};
use alloc::string::*;
use alloc::boxed::Box;
use core::mem;

pub use {u256::U256, address::Address};

enum WbiTypes {
    BOOL = 0, // 0
    I64 = 1,  // 1
    U64 = 2, //  2 BN
    F64 = 3, //
    STRING = 4, // 4 string
    BYTES = 5, // 5
    ADDRESS = 6, // 6
    U256 = 7
}

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
        fn panic(info: &core::panic::PanicInfo) -> !{
            log(&format!("{:?}", info));
            unsafe { core::arch::wasm32::unreachable() }
        }        
    };
}

extern "C" {
    pub fn _log(a: u64); 
    pub fn _context(t: u64, a: u64) -> u64;
}

pub fn log(s: &str) {
    unsafe {
        let raw_cloned = String::from_raw_parts(
        s.as_ptr() as *mut u8,
      s.len(),
    s.len()
        );
        _log(forget(raw_cloned) as u64)
    }
}

// allocate memory, return as raw pointer
#[no_mangle]
pub fn __malloc(size: u64) -> u64 {
    let bytes = vec![0u8; size as usize];
    forget_bytes(bytes)
}

// convert t to raw, 
#[inline]
fn forget_bytes(t: Vec<u8>) -> u64 {
    let raw = t.as_ptr();
    let ret = (raw as usize) as u64;
    mem::forget(t);
    ret
}

// restore Vec<u8> from raw pointer and length
#[inline]
fn remember_bytes(ptr: u64, size: u64) -> Vec<u8> {
    unsafe {
        let raw = ptr as *mut u8;
        Vec::from_raw_parts(raw, size as usize, size as usize)
    }
}

#[inline]
fn remember<T>(p: u64) -> T {
    unsafe {
        let b = Box::from_raw(p as *mut T);
        return *b;
    }
}

#[inline]
fn forget<T>(d: T) -> *mut T {
    let r = Box::new(d);
    Box::leak(r)
}

#[inline]
pub fn ret<T>(d: T) -> &'static T {
    let r = Box::new(d);
    Box::leak(r)
}


/// convert bytes view to rust type
#[no_mangle]
pub unsafe fn __change_t(t: u64, ptr: u64, size: u64) -> u64 {
    let v = remember_bytes(ptr, size);
    // string
    if t == WbiTypes::STRING as u64 {
        let s = String::from_utf8_unchecked(v);
        return forget(s) as u64;
    }

    if t == WbiTypes::BYTES as u64 {
        return forget(v) as u64;
    }

    if t == WbiTypes::U256 as u64 {
        let u = U256::new(v);
        return forget(u) as u64;
    }

    if t == WbiTypes::ADDRESS as u64 {
        let addr = Address::new(v);
        return forget(addr) as u64;
    }    

    return 0;
}

/// __peek will convert rust type to bytes view, this function is called by host
#[no_mangle]
pub fn __peek(ptr: u64, t: u64) -> u64 {
    if t == WbiTypes::STRING as u64 {
        let p: String = remember(ptr);
        let (x, y) = (p.as_ptr() as u64, p.len());
        mem::forget(p);
        return (x << 32) | (y as u64);
    }

    if t == WbiTypes::BYTES as u64 {
        let p: Vec<u8> = remember(ptr);
        let (x, y) = (p.as_ptr() as u64, p.len());
        mem::forget(p);
        return (x << 32) | (y as u64);
    }    

    if t == WbiTypes::U256 as u64 {
        let p: U256 = remember(ptr);
        let (x, y) = p.__peek();
        mem::forget(p);
        return (x << 32) | (y as u64);
    }

    if t == WbiTypes::ADDRESS as u64 {
        let p: Address = remember(ptr);
        let (x, y) = p.__peek();
        mem::forget(p);
        return (x << 32) | (y as u64);
    }    
    return 0;
}
