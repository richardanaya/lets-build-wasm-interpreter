#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}
#[no_mangle]
fn run(ptr: *mut u8, len: usize) -> f64 {
    let rebuilt = unsafe { Vec::from_raw_parts(ptr, len, len) };
    rebuilt.len() as f64
}
