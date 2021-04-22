use alloc::vec::Vec;
use crate::remember_bytes;
use hex::ToHex;
use alloc::string::*;

const ADDRESS_SIZE: usize = 20;

#[derive(Eq)]
pub struct Address {
    data: Vec<u8>
}

impl ToString for Address {
    fn to_string(&self) -> String {
        self.data.encode_hex()
    }
}

impl Default for Address {
    fn default() -> Address {
        Address {
            data: vec![0u8; ADDRESS_SIZE]
        }
    }
}


impl rlp::Decodable for Address {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        // 1. shouldn't starts with zero
		rlp.decoder().decode_value(|bytes| {
			if bytes.len() != ADDRESS_SIZE{
                return Err(rlp::DecoderError::Custom("invalid address size"));
            }
       
            return Ok(Address::new(bytes.to_vec()))
		})        
    }
}

impl rlp::Encodable for Address {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.encoder().encode_value(&self.data);
    }
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