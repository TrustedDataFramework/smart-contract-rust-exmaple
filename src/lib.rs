#![no_std]
#![feature(default_alloc_error_handler)]

extern crate wee_alloc;

#[macro_use]
extern crate wbi;

use_wee_alloc!();  

use alloc::{vec::Vec};
use alloc::string::*;


pub use wbi::{__change_t, __malloc, __peek, log, ret};

#[no_mangle]
pub fn init(a: String, b: String, c: Vec<u8>) -> &'static Vec<u8>{
    log(&a);
    log(&b);


    let r = vec![0xfeu8, 0xfeu8];
    ret(r)
}
