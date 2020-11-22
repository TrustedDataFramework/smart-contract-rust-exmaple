extern "C" {
    pub fn _log(a: u64);
}

unsafe fn log(msg: &str) {
    let d: Data = Data { ptr: msg.as_ptr(), len: msg.len() as u32 };
    _log(&d as *const _ as u64);
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
    pub unsafe fn string(&self) -> String {
        let mut bytes: Vec<u8> = Vec::with_capacity(self.len as usize);
        bytes.set_len(self.len as usize);
        for x in 0..self.len{
            bytes[x as usize] = *(self.ptr.offset(x as isize))
        }
        return String::from_utf8_unchecked(bytes);
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
pub unsafe extern fn init(s: &Data) {
    log(s.string().as_str());
}
