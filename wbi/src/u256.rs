use core::{ops::{Add, Sub, Mul, Div, Rem}, str::FromStr};
use core::cmp::{PartialEq, Eq, PartialOrd, Ordering};
use alloc::{vec::Vec};
use crate::{forget, remember, log, remember_bytes};
use alloc::string::*;

const chars: &'static str = "0123456789";

extern "C" {
    pub fn _u256(op: u64, left: u64, right: u64) -> u64;
}

impl Default for U256 {
    fn default() -> U256 {
        U256::zero()
    }
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
                if *i != 0 {
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

// overflow check
macro_rules! impl_op {
    ($tr: ident, $fn: ident, $op: expr, $overflow: ident) => {
        impl<'a> $tr for &'a U256 {
            type Output = U256;    
        
            fn $fn(self, rhs: &'a U256) -> U256 {
                let l = self.raw_clone();
                let r = rhs.raw_clone();       
                let p = unsafe {
                    _u256($op as u64, forget(l) as u64, forget(r) as u64)
                };
                let o: U256 = remember(p);
                if $overflow(self, rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }
                o
            }
        }

        impl<'a> $tr<U256> for &'a U256 {
            type Output = U256;    
        
            fn $fn(self, rhs: U256) -> U256 {
                let l = self.raw_clone();
                let r = rhs.raw_clone();  
                let p = unsafe {
                    _u256($op as u64, forget(l) as u64, forget(r) as u64)
                };
                let o = remember(p);
                if $overflow(self, &rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }                
                o
            }
        }        
        
        impl<'a> $tr<&'a U256> for U256 {
            type Output = U256;  
        
            fn $fn(self, rhs: &'a U256) -> U256 {
                let l = self.raw_clone();
                let r = rhs.raw_clone();       
                let p = unsafe {
                    _u256($op as u64, forget(l) as u64, forget(r) as u64)
                };
                let o: U256 = remember(p);
                if $overflow(&self, rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }                   
                o
            }
        }     
        
        impl $tr for U256 {
            type Output = U256;  
        
            fn $fn(self, rhs: U256) -> U256 {
                let l = self.raw_clone();
                let r = rhs.raw_clone();                   
                let p = unsafe {
                    _u256($op as u64, forget(l) as u64, forget(r) as u64)
                };
                let o: U256 = remember(p);
                if $overflow(&self, &rhs, &o) {
                    panic!("math overflow for op {}", $op as u8);
                }                   
                o
            }
        }          
    };
}

fn add_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    out < left || out < right
}

fn sub_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    right > left
}

fn mul_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    if left.is_zero() {
        false
    } else {
        &(out / left) != right
    }
}

fn div_over_flow(left: &U256, right: &U256, out: &U256) -> bool {
    right.is_zero()
}

impl_op!(Add, add, Op::SUM, add_over_flow);
impl_op!(Sub, sub, Op::SUB, sub_over_flow);
impl_op!(Mul, mul, Op::MUL, mul_over_flow);
impl_op!(Div, div, Op::DIV, div_over_flow);
impl_op!(Rem, rem, Op::MOD, div_over_flow);


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

    pub fn max() -> U256 {
        U256 {
            data: vec![0xffu8; 32]
        }
    }

    pub fn pow(&self, o: u64) -> U256 {
        let mut ret = U256::one();
    
        for _ in 0..o{
            ret = &ret * self
        }
        ret      
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
