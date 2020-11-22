#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::ContextType::MSG_SENDER;
use std::fmt::Write;

extern "C" {
    pub fn _log(a: u64);
    pub fn __log(a: u64);
    pub fn _nop(a: u64);
    pub fn _context(a: u64, b: u64) -> u64;
}

fn log(msg: &str) {
    let d: Data = Data { ptr: msg.as_ptr(), len: msg.len() as u32 };
    unsafe {
        _log(&d as *const _ as u64);
    }
}

struct Msg {
    sender: Vec<u8>
}


impl Msg {
    pub fn new() -> Self {
        let ptr = unsafe { _context(MSG_SENDER as u64, 0) };
        let p: *const Data = ptr as usize as *const _;
        return unsafe {
            Msg { sender: (*p).bytes() }
        };
    }
}


fn toHex(bytes: &[u8]) -> String {
    let mut s = String::new();
    for &byte in bytes {
        write!(&mut s, "{:x} ", byte).expect("Unable to write");
    }
    return s;
}

enum ContextType {
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

// 防止指针被 rust 回收
fn ret<T>(d: &T) -> *const T {
    let r = d as *const T;
    unsafe {
        _nop(r as u64);
    }
    return r;
}

enum AbiDataType {
    BOOL,
    // 0
    I64,
    // 1
    U64,
    //  2 BN
    F64,
    STRING,
    // 3 string
    BYTES,
    // 4
    ADDRESS,
    // 5
    U256, // 6
}

// WBI 在 rust 中的实现
// 因为 rust 没有内存管理，需要用一个结构体来保存字节流的长度
#[repr(C)]
#[no_mangle]
pub struct Data {
    ptr: *const u8,
    len: u32,
}

impl Data {
    pub fn from_str(s: &str) -> Self {
        return Data {
            ptr: s.as_ptr(),
            len: s.len() as u32,
        };
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        return Data {
            ptr: bytes.as_ptr(),
            len: bytes.len() as u32,
        };
    }

    pub fn bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len as usize);
        unsafe {
            bytes.set_len(self.len as usize);
        }
        for x in 0..self.len {
            bytes[x as usize] = unsafe {
                *(self.ptr.offset(x as isize))
            }
        }
        return bytes;
    }
    pub fn string(&self) -> String {
        unsafe {
            return String::from_utf8_unchecked(self.bytes());
        }
    }
}

#[no_mangle]
pub unsafe extern fn __malloc(size: u64) -> u64 {
    let mut bytes: Vec<u8> = Vec::with_capacity(size as usize);
    bytes.set_len(size as usize);
    return bytes.as_ptr() as u64;
}

#[no_mangle]
pub unsafe extern fn __change_t(t: u64, ptr: u64, size: u64) -> u64 {
    let d = Data { ptr: ptr as usize as *const u8, len: size as u32 };
    return &d as *const _ as u64;
}


#[no_mangle]
pub unsafe extern fn __peek(ptr: u64, t: u64) -> u64 {
    let p: *const Data = ptr as usize as *const _;
    let len = (*p).len;
    return (((*p).ptr as u64) << 32) | (len as u64);
}

#[no_mangle]
pub unsafe extern fn init() {
    // 打印 msg.sender()
    let m = Msg::new();
    log(toHex(m.sender.as_slice()).as_str());

    // 打印 sender 的 balance
    let dp = Data::from_bytes(m.sender.as_slice());
    let db: *const Data = _context(ContextType::ACCOUNT_BALANCE as u64, &dp as *const _ as u64)
        as usize as *const _;
    let d: &Data = &(*db);
    log(toHex(d.bytes().as_slice()).as_str());
}

// 合约内字符串拼接 返回类型必须是指针
#[no_mangle]
pub extern fn concat_world(s: &Data) -> *const Data {
    let mut str = s.string();
    str.push_str(" world!");
    let d = Data::from_str(str.as_str());
    return ret(&d);
}
