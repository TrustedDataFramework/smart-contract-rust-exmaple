#![allow(dead_code)]
#![allow(non_camel_case_types)]

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
    bytes.as_ptr() as u64
}

#[no_mangle]
pub unsafe fn __change_t(t: u64, ptr: u64, size: u64) -> u64{
    let p = ptr as *const u8;
    let mut bytes: Vec<u8> = Vec::with_capacity(size as usize);
    bytes.set_len(size as usize);
    for i in 0..size {
        bytes[i as usize] = *(p.offset(i as isize));
    }
    let s = String::from_utf8_unchecked(bytes);
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
