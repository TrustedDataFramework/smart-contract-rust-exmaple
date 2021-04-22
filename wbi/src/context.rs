extern "C" {
    pub fn _context(t: u64, a: u64) -> u64;
}

use crate::{forget, log, remember};
use crate::address::Address;
use crate::u256::U256;
use alloc::vec::Vec;

enum CONTEXT_TYPE {
    HEADER_PARENT_HASH,
    HEADER_CREATED_AT,
    HEADER_HEIGHT,
    TX_TYPE,
    TX_CREATED_AT,
    TX_NONCE,
    TX_ORIGIN,
    TX_GAS_PRICE,
    TX_AMOUNT,
    TX_TO,
    TX_SIGNATURE,
    TX_HASH,
    CONTRACT_ADDRESS,
    CONTRACT_NONCE,
    CONTRACT_CREATED_BY,
    ACCOUNT_NONCE,
    ACCOUNT_BALANCE,
    MSG_SENDER,
    MSG_AMOUNT,
    CONTRACT_CODE,
    CONTRACT_ABI,
}

pub fn this() -> Address {
    unsafe {
        remember(_context(CONTEXT_TYPE::CONTRACT_ADDRESS as u64, 0))
    }
}

lazy_static! {
    pub static ref msg: Msg = {
        Msg::new()
    };
    pub static ref block: Block = {
        Block::new()
    };    
    pub static ref tx: Transaction = {
        Transaction::new()
    };   

}

pub struct Msg {
    pub sender: Address,
    pub amount: U256
}

impl Msg {
    fn new() -> Msg {
        unsafe {
            Msg {
                sender: remember(_context(CONTEXT_TYPE::MSG_SENDER as u64, 0)),
                amount: remember(_context(CONTEXT_TYPE::MSG_AMOUNT as u64, 0)),
            }
        }
    }
}

pub struct Block {
    pub parent_hash: Vec<u8>,
    pub created_at: u64,
    pub height: u64,
}

pub struct Transaction {
    pub tx_type: u8,
    pub created_at: u64,
    pub nonce: u64,
    pub origin: Address,
    pub gas_price: U256,
    pub amount: U256,
    pub to: Address,
    pub signature: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Transaction {
    pub fn new() -> Transaction {
        unsafe {
            Transaction {
                tx_type: _context(CONTEXT_TYPE::TX_TYPE as u64, 0) as u8,
                created_at: _context(CONTEXT_TYPE::TX_CREATED_AT as u64, 0),
                nonce: _context(CONTEXT_TYPE::TX_NONCE as u64, 0),
                origin: remember(_context(CONTEXT_TYPE::TX_ORIGIN as u64, 0)),
                gas_price: remember(_context(CONTEXT_TYPE::TX_GAS_PRICE as u64, 0)),
                amount: remember(_context(CONTEXT_TYPE::TX_AMOUNT as u64, 0)),
                to: remember(_context(CONTEXT_TYPE::TX_TO as u64, 0)),
                signature: remember(_context(CONTEXT_TYPE::TX_SIGNATURE as u64, 0)),
                hash: remember(_context(CONTEXT_TYPE::TX_HASH as u64, 0)),
            }
        }
    }
}

impl Block {
    pub fn new() -> Block {
        unsafe {
            Block {
                parent_hash: remember(_context(CONTEXT_TYPE::HEADER_PARENT_HASH as u64, 0)),
                created_at: _context(CONTEXT_TYPE::HEADER_CREATED_AT as u64, 0),
                height: _context(CONTEXT_TYPE::HEADER_HEIGHT as u64, 0),
            }
        }
    }
}

impl Address {
    pub fn balance(&self) -> U256 {
        let ptr = unsafe {
            _context(
                CONTEXT_TYPE::ACCOUNT_BALANCE as u64, 
                forget(self.raw_clone()) as u64
            )
        };
        return remember(ptr);
    }

    pub fn nonce(&self) -> u64 {
        let ptr = unsafe {
            _context(
                CONTEXT_TYPE::ACCOUNT_NONCE as u64, 
                forget(self.raw_clone()) as u64
            )
        };
        return ptr;
    }
}