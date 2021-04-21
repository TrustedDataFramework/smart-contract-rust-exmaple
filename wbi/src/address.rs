use alloc::vec::Vec;
use crate::remember_bytes;
#[derive(Eq)]
pub struct Address {
    data: Vec<u8>
}

impl PartialEq for Address {
    fn eq(&self, other: &Address) -> bool {
        for i in 0..self.data.len() {
            if self.data[i] != other.data[i] {
                return false;
            }            
        }

        return true;      
    }    
}

impl Address {
    pub fn new(v: Vec<u8>) -> Address {
        Address {
            data: v
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    pub(crate) fn __peek(&self) -> (u64, u64){
        (self.data.as_ptr() as u64, self.data.len() as u64)
    }        

    pub(crate) fn raw_clone(&self) -> Address {
        let (x, y) = (self.data.as_ptr() as u64, self.data.len() as u64);
        let v = remember_bytes(x, y);
        Address {
            data: v
        }
    }
}