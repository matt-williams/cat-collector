use std::mem;

mod ffi {
    extern "C" {
        pub fn console_log(msg: *const u8, msg_len: u32);
        pub fn display_add(msg: *const u8, msg_len: u32);
        pub fn timeout_set(delay: u32);
        pub fn math_random() -> f32;
    }
}

#[allow(dead_code)]
pub fn log<S: AsRef<str>>(msg: S) {
    let msg = msg.as_ref();
    unsafe {
        ffi::console_log(msg.as_ptr(), msg.len() as u32);
    }
}

pub fn display<S: AsRef<str>>(msg: S) {
    let msg = msg.as_ref();
    unsafe {
        ffi::display_add(msg.as_ptr(), msg.len() as u32);
    }
}

pub fn timeout_set(delay: u32) {
    unsafe {
        ffi::timeout_set(delay);
    }
}

pub fn math_random() -> f32 {
    unsafe {
        ffi::math_random()
    }
}

#[no_mangle]
pub fn alloc(len: u32) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(len as usize);
    let ptr = vec.as_mut_ptr();
    mem::forget(vec);
    ptr
}

#[no_mangle]
pub fn free(ptr: *mut u8) {
    // Should probably pass correct len, but it doesn't seem to matter, and
    // it's easier if we don't have to track!
    unsafe{Vec::<u8>::from_raw_parts(ptr, 0, 1)};
}
