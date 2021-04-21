use core::{ops::{Add, Sub, Mul, Div, Rem}, str::FromStr};
use core::cmp::{PartialEq, Eq, PartialOrd, Ordering};
use alloc::{vec::Vec};
use crate::{forget, remember, log, remember_bytes};
use alloc::string::*;

const chars: &'static str = "0123456789";

extern "C" {
    pub fn _u256(op: u64, left: u64, right: u64) -> u64;
}

impl FromStr for U256 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, &'static str> {
        let mut r = U256::zero();
        let mut base = U256::one();
        let ten = U256::from(10 as u64);

        for c in s.as_bytes().iter().rev() {
            let n = *c - b'0';
            let n = U256::from(n as u64);
            r = &r + &(&n * &base);
            base = &base * &ten;
        }
        return Ok(r);   
    }
}

enum Op {
    SUM = 0,
    SUB = 1,
    MUL = 2,
    DIV = 3,
    MOD = 4   
}

impl rlp::Decodable for U256 {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        // 1. shouldn't starts with zero
		rlp.decoder().decode_value(|bytes| {
			for i in bytes.iter() {
                if *i == 0 {
                    break;
                } else {
                    return Err(rlp::DecoderError::RlpDataLenWithZeroPrefix)
                }
            }

            return Ok(U256::new(bytes.into()))
		})        
    }
}

impl rlp::Encodable for U256 {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.encoder().encode_value(&self.data);
    }
}


#[derive(Clone, Eq, Ord)]
pub struct U256 {
    data: Vec<u8>,
}

impl PartialEq for U256 {
    fn eq(&self, other: &U256) -> bool {
        if self.data.len() > other.data.len() {
            return false;
        }
        if self.data.len() < other.data.len() {
            return false;
        }        

        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }            
        }

        return true;      
    }    
}

impl PartialOrd for U256 {

    fn partial_cmp(&self, other: &U256) -> Option<Ordering> {
        if self.data.len() > other.data.len() {
            return Some(Ordering::Greater);
        }
        if self.data.len() < other.data.len() {
            return Some(Ordering::Less);
        }        

        for i in 0..self.data.len() {
            if self.data[i] > other.data[i] {
                return Some(Ordering::Greater);
            }
            if self.data[i] < other.data[i] {
                return Some(Ordering::Less);
            }            
        }

        Some(Ordering::Equal)
    }
}


impl ToString for U256 {
    fn to_string(&self) -> String {
        let mut ret = String::new();

        let base: U256 = 10u64.into();

        let mut n = self.clone();

        if n.is_zero() {
            return "0".to_string()
        }
        while !n.is_zero() {
            let div = &n / &base;
            let m = &n % &base;
            n = div;
            ret.insert(0, chars.as_bytes()[m.u64() as usize] as char);
        }
        return ret
    }
}


impl<'a> Add for &'a U256 {
    type Output = U256;    

    fn add(self, rhs: &'a U256) -> U256 {
        let l = self.raw_clone();
        let r = rhs.raw_clone();       
        let o = unsafe {
            _u256(Op::SUM as u64, forget(l) as u64, forget(r) as u64)
        };
        remember(o)
    }
}

impl<'a> Sub for &'a U256 {
    type Output = U256;

    fn sub(self, rhs: &'a U256) -> U256 {
        let l = self.raw_clone();
        let r = rhs.raw_clone();   

        let o = unsafe {
            _u256(Op::SUB as u64, forget(l) as u64, forget(r) as u64)
        };
        remember(o)
    }
}

impl<'a> Mul for &'a U256 {
    type Output = U256;

    fn mul(self, rhs: &'a U256) -> U256 {
        let l = self.raw_clone();
        let r = rhs.raw_clone();
        let o = unsafe {
            _u256(Op::MUL as u64, forget(l) as u64, forget(r) as u64)
        };
        remember(o)
    }
}

impl<'a> Div for &'a U256 {
    type Output = U256;

    fn div(self, rhs: &'a U256) -> U256 {
        let l = self.raw_clone();
        let r = rhs.raw_clone();
        let o = unsafe {
            _u256(Op::DIV as u64, forget(l) as u64, forget(r) as u64)
        };
        remember(o)
    }
}


impl<'a> Rem for &'a U256 {
    type Output = U256;

    fn rem(self, rhs: &'a U256) -> U256 {
        let l = self.raw_clone();
        let r = rhs.raw_clone();
        let o = unsafe {
            _u256(Op::MOD as u64, forget(l) as u64, forget(r) as u64)
        };
        remember(o)
    }
}

impl From<u64> for U256 {
    fn from(o: u64) -> U256 {
        let bytes: [u8; 8] = o.to_be_bytes();
        // the last zero bytes
        let mut i: usize = 8;
        let mut j: usize = 0;
        for u in bytes.iter() {
            if *u != 0 {
                i = j;
                break;
            } 
            j += 1;
        }

        let mut v = Vec::with_capacity(8 - j);
        v.extend_from_slice(&bytes[i..]);
        U256::new(v)
    }
}

impl U256 {
    pub(crate) fn raw_clone(&self) -> U256{
        let (x, y) = (self.data.as_ptr() as u64, self.data.len() as u64);
        let v = remember_bytes(x, y);
        U256 {
            data: v
        }
    }

    pub fn new(v: Vec<u8>) -> U256 {
        U256 {
            data: v
        }        
    }  

    pub(crate) fn __peek(&self) -> (u64, u64){
        (self.data.as_ptr() as u64, self.data.len() as u64)
    }    


    pub fn zero() -> U256 {
        U256 {
            data: Vec::new()
        }
    }

    pub fn one() -> U256 {
        U256 {
            data: vec![1u8]
        }
    }

    pub fn u64(&self) -> u64 {
        let mut v = [0u8; 8];
        let i = self.data.len();
        (&mut v[8-i..]).copy_from_slice(&self.data);
        u64::from_be_bytes(v)
    }


    pub fn is_zero(&self) -> bool {
        self.data.len() == 0
    }
}
