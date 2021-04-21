use alloc::vec::Vec;
use crate::{forget, remember};


// get vector from fat pointer
// should forget 
macro_rules! to_vec {
    ($slice: expr) => {
        unsafe {
            Vec::from_raw_parts($slice.as_ptr() as *mut u8, $slice.len(), $slice.len())
        }
    };
}

enum Op {
    SET = 0, GET = 1, REMOVE = 2, HAS = 3
}

pub fn insert(key: &[u8], value: &[u8]) {
    let k = to_vec!(key);
    let v = to_vec!(value);
    unsafe {
        _db(Op::SET as u64, forget(k) as u64, forget(v) as u64);
    }
}

pub fn contains_key(key: &[u8]) -> bool {
    let k = to_vec!(key);
    unsafe {
        _db(Op::HAS as u64, forget(k) as u64, 0) != 0
    }
}

pub fn get(key: &[u8]) -> Option<Vec<u8>>{
    let k = to_vec!(key);
    if !contains_key(key) {
        forget(k);
        Option::None
    } else {
        let p = unsafe { _db(Op::GET as u64, forget(k) as u64, 0) };
        remember(p)
    }
}

pub fn remove(key: &[u8]) {
    let k = to_vec!(key);
    unsafe { _db(Op::REMOVE as u64, forget(k) as u64, 0) };
}

extern "C" {
    pub fn _db(op: u64, left: u64, right: u64) -> u64;
}
