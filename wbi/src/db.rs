use alloc::vec::Vec;
use crate::{forget, remember};
use alloc::string::*;
use core::marker::PhantomData;

pub struct Store<K, V> {
    prefix: String,
    phantom_0: PhantomData<K>,
    phantom_1: PhantomData<V>,
}

macro_rules! concat {
    ($l: expr, $r: expr) => {
        {
            let mut v = Vec::with_capacity($l.len() + $r.len());
            v.extend_from_slice($l);
            v.extend_from_slice($r);
            v
        }
    };
}
impl<K: rlp::Encodable + rlp::Decodable, V: rlp::Encodable + rlp::Decodable> Store<K, V> {
    pub fn new(s: &str) -> Store<K, V> {
        Store {
            prefix: s.to_string(),
            phantom_0: PhantomData::default(),
            phantom_1: PhantomData::default(),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let encoded = rlp::encode(key);
        let k = concat!(&encoded, self.prefix.as_bytes());
        get(&k)
            .map(
                |x| rlp::decode(&x).unwrap()
            )
    }

    pub fn contains_key(&self, key: &K) -> bool {
        let k = concat!(&rlp::encode(key), self.prefix.as_bytes());
        contains_key(&k)
    }


    pub fn insert(&self, key: &K, val: &V) {
        let encoded = rlp::encode(key);
        let k = concat!(&encoded, self.prefix.as_bytes());
        insert(&k, &rlp::encode(val));
    }

    pub fn remove(&self, key: &K) {
        let encoded = rlp::encode(key);
        let k = concat!(&encoded, self.prefix.as_bytes());
        remove(&k)
    }    
}



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

pub struct Globals;

impl Globals {
    pub fn get<V: rlp::Decodable>(name: &str) -> Option<V> {
        get(name.as_bytes())
        .map(
            |x| rlp::decode(&x).unwrap()
        )   
    }
    
    pub fn insert<V: rlp::Encodable>(key: &str, value: &V) {
        insert(key.as_bytes(), &rlp::encode(value))        
    }
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
