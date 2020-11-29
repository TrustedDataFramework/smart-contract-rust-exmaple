#![allow(dead_code)]
#![allow(non_camel_case_types)]

// 本质上 Box<String> 和 &String 是一样的
extern "C" {
    pub fn _log(a: u64);
    pub fn __log(a: u64);
    pub fn _context(t: u64, a: u64) -> u64;
}

pub struct Address {
    vec: Box<Vec<u8>>
}

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

struct Header {
    parent_hash: Vec<u8>,
    created_at: u64,
    height: u64,
}

struct Transaction {
    tx_type: u8,
    created_at: u64,
    nonce: u64,
    origin: Address,
    gas_price: U256,
    amount: U256,
    to: Address,
    signature: Vec<u8>,
    hash: Vec<u8>,
}

impl Transaction {
    fn new() -> Transaction {
        unsafe {
            Transaction {
                tx_type: _context(CONTEXT_TYPE::TX_TYPE as u64, 0) as u8,
                created_at: _context(CONTEXT_TYPE::TX_CREATED_AT as u64, 0),
                nonce: _context(CONTEXT_TYPE::TX_CREATED_AT as u64, 0),
                origin: get_type(_context(CONTEXT_TYPE::TX_ORIGIN as u64, 0)),
                gas_price: get_type(_context(CONTEXT_TYPE::TX_GAS_PRICE as u64, 0)),
                amount: get_type(_context(CONTEXT_TYPE::TX_AMOUNT as u64, 0)),
                to: get_type(_context(CONTEXT_TYPE::TX_TO as u64, 0)),
                signature: get_type(_context(CONTEXT_TYPE::TX_SIGNATURE as u64, 0)),
                hash: get_type(_context(CONTEXT_TYPE::TX_HASH as u64, 0)),
            }
        }
    }
}


impl Header {
    fn new() -> Header {
        unsafe {
            Header {
                parent_hash: get_type(_context(CONTEXT_TYPE::HEADER_PARENT_HASH as u64, 0)),
                created_at: _context(CONTEXT_TYPE::HEADER_CREATED_AT as u64, 0),
                height: _context(CONTEXT_TYPE::HEADER_HEIGHT as u64, 0),
            }
        }
    }
}

impl Address {
    fn clone(&self) -> Address {
        Address {
            vec: self.vec.clone()
        }
    }

    fn balance(&self) -> U256 {
        let ptr = unsafe {
            _context(CONTEXT_TYPE::ACCOUNT_BALANCE as u64, ret(self.clone()) as u64)
        };
        return get_type(ptr);
    }

    fn nonce(&self) -> u64 {
        let ptr = unsafe {
            _context(CONTEXT_TYPE::ACCOUNT_NONCE as u64, ret(self.clone()) as u64)
        };
        return ptr;
    }
}

pub struct U256 {
    vec: Box<Vec<u8>>
}

impl U256 {
    fn clone(&self) -> U256 {
        U256 {
            vec: self.vec.clone()
        }
    }
}

fn get_type<T>(p: u64) -> T {
    unsafe {
        let b = Box::from_raw(p as *mut T);
        return *b;
    }
}


fn _parent_hash() -> Vec<u8> {
    unsafe {
        let p = _context(0, 0);
        return get_type(p);
    }
}

fn log(msg: &str) {
    unsafe {
        _log(ret(String::from(msg)) as u64);
    }
}

// 防止 rust 内存回收
pub fn ret<T>(d: T) -> *mut T {
    let r = Box::new(d);
    unsafe {
        Box::leak(r)
    }
}

#[no_mangle]
pub unsafe fn __malloc(size: u64) -> u64 {
    let mut bytes: Vec<u8> = Vec::with_capacity(size as usize);
    bytes.set_len(size as usize);
    bytes.leak().as_ptr() as u64
}

#[no_mangle]
pub unsafe fn __change_t(t: u64, ptr: u64, size: u64) -> u64 {
    let v: Vec<u8> = Vec::from_raw_parts(ptr as *mut _, size as usize, size as usize);
    // string
    if t == 4 {
        let s = String::from_utf8_unchecked(v);
        return ret(s) as u64;
    }
    // bytes
    if t == 5 {
        return ret(v) as u64;
    }
    // address
    if t == 6 {
        let a = Address { vec: Box::new(v) };
        return ret(a) as u64;
    }
    // u256
    if t == 7 {
        let u = U256 { vec: Box::new(v) };
        return ret(u) as u64;
    }
    return 0;
}


#[no_mangle]
pub unsafe fn __peek(ptr: u64, t: u64) -> u64 {
    // string
    if t == 4 {
        let s: String = get_type(ptr);
        let bytes = s.into_bytes();
        let l = bytes.leak();
        return ((l.as_ptr() as u64) << 32) | l.len() as u64;
    }
    // bytes
    if t == 5 {
        let v: Vec<u8> = get_type(ptr);
        let l = v.leak();
        return ((l.as_ptr() as u64) << 32) | l.len() as u64;
    }
    // address
    if t == 6 {
        let a: Address = get_type(ptr);
        let l = (*a.vec).leak();
        return ((l.as_ptr() as u64) << 32) | l.len() as u64;
    }
    // u256
    if t == 7 {
        let u: U256 = get_type(ptr);
        let l = (*u.vec).leak();
        return ((l.as_ptr() as u64) << 32) | l.len() as u64;
    }
    return 0;
}

#[no_mangle]
pub unsafe fn init(a: &String, b: &String) -> *mut String {
    let h = Header::new();
    log("当前高度");
    log(h.height.to_string().as_str());
    log("父区块哈希值");
    log(&format!("{:02X?}", h.parent_hash).as_str());
    log("当前时间戳");
    log(h.created_at.to_string().as_str());

    let tx = Transaction::new();
    log("事务类型");
    log(tx.tx_type.to_string().as_str());
    log("事务时间戳");
    log(tx.created_at.to_string().as_str());
    log("事务 nonce");
    log(tx.nonce.to_string().as_str());
    log("事务 origin");
    log(&format!("{:02X?}", tx.origin.vec).as_str());
    log("事务 to");
    log(&format!("{:02X?}", tx.to.vec).as_str());
    log("事务 签名");
    log(&format!("{:02X?}", tx.signature).as_str());
    log("事务 哈希值");
    log(&format!("{:02X?}", tx.hash).as_str());
    ret(a.to_string() + b.as_str())
}

#[no_mangle]
pub fn echo0(b: &Vec<u8>) -> *mut Vec<u8> {
    ret(b.to_vec())
}

#[no_mangle]
pub fn echo1(c: &Address) -> *mut Address {
    ret(c.clone())
}

#[no_mangle]
pub fn echo2(c: &U256) -> *mut U256 {
    ret(c.clone())
}

#[no_mangle]
pub fn balance(a: &Address) -> *mut U256 {
    ret(a.balance())
}
