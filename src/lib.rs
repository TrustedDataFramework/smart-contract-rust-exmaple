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

#[macro_use]
extern crate lazy_static;

extern "C" {
    pub fn __log(a: u64); 
}

fn log_u64(x: u64) {
    unsafe {
        __log(x);
    }
}

mod persist;
mod types;
mod utils;

use hex::ToHex;
use lazy_static::lazy_static;
pub use wbi::{__change_t, __malloc, __peek, log, ret};
use wbi::db;
use wbi::{Address, U256 };
use wbi::context;
use utils::RayMath;

#[no_mangle]
pub fn init(a: Address, b: &U256) -> &'static U256{
    let r = RayMath::half_ray();
    let s = utils::decimal(r, 27);
    log(&s);

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

