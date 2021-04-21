#![no_std]
#![feature(default_alloc_error_handler)]

extern crate wee_alloc;

#[macro_use]
extern crate wbi;

use_wee_alloc!();  

use alloc::{vec::Vec};
use alloc::string::*;
extern crate hex;
extern crate rlp_derive;

extern "C" {
    pub fn __log(a: u64); 
}

fn log_u64(x: u64) {
    unsafe {
        __log(x);
    }
}

mod types;

use hex::ToHex;
pub use wbi::{__change_t, __malloc, __peek, log, ret};
use wbi::db;
use wbi::{Address, U256 };
use wbi::context;

#[no_mangle]
pub fn init(a: Address, b: &U256) -> &'static U256{
    panic!("paniccccc");
    let s = "314159265358979323846264338327950288419716939937510";

    let n: U256 = s.parse().unwrap();
    log(&n.to_string());
    ret(U256::one())
}

#[no_mangle]
pub fn insert(k: Vec<u8>, v: Vec<u8>) {
    db::insert(&k, &v);
}

#[no_mangle]
pub fn get(k: Vec<u8>) -> &'static Vec<u8>{
    let r = db::get(&k)
        .unwrap_or(Vec::new());
    
    ret(r)
}

