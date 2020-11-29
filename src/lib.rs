#![allow(dead_code)]
#![allow(non_camel_case_types)]

// 本质上 Box<String> 和 &String 是一样的
extern "C" {
    pub fn _log(a: u64);
    pub fn __log(a: u64);
}

fn log(msg: &String) {
    unsafe {
        _log(msg as *const _ as u64);
    }
}

// 防止 rust 内存回收
pub fn ret<T>(d: T) -> *mut T {
    let r = Box::new(d);
    unsafe{
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
pub unsafe fn __change_t(t: u64, ptr: u64, size: u64) -> u64{
    let v: Vec<u8> = Vec::from_raw_parts(ptr as *mut _, size as usize, size as usize);
    let s = String::from_utf8_unchecked(v);
    ret(s) as u64
}


#[no_mangle]
pub unsafe fn __peek(ptr: u64, t: u64) -> u64 {
    let sp: *mut String = ptr as *mut _;
    let s = Box::from_raw(sp);
    let bytes = s.into_bytes();
    let l = bytes.leak();
    ((l.as_ptr() as u64) << 32) | l.len() as u64
}

#[no_mangle]
pub fn init(a: &String, b: &String) -> *mut String{
    let mut s = String::from(a);
    s.push_str(b.as_str());
    ret(s)
}
