#![no_std]
#![feature(core_intrinsics, lang_items, alloc_error_handler)]
// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}

// Need to provide an allocation error handler which just aborts
// the execution with trap.
#[alloc_error_handler]
#[no_mangle]
pub extern "C" fn oom(_: ::core::alloc::Layout) -> ! {
    unsafe {
        ::core::intrinsics::abort();
    }
}
extern crate alloc;
use alloc::string::{String,ToString};
use alloc::vec::Vec;
use watson::*;

extern crate wee_alloc;

extern "C" {
    fn _log(msg: *const u8);
}

fn log(msg: &str) {
    let mut s = msg.to_string();
    s.push_str("\0");
    unsafe { _log(s.as_ptr()) }
}

fn load_and_run_main(wasm_bytes: &[u8]) -> Result<f64, String> {
    let program = watson::parse(&wasm_bytes)?;
    let main_function = program.find_exported_function("main")?;
    let main_code = program.find_code_block(main_function.index)?;
    if let Instruction::I32Const(num) = main_code.code_expression[0] {
        Ok(num as f64)
    } else {
        Err("Interpreter can't do anything else yet.".to_string())
    }
}

#[no_mangle]
fn run(ptr: *mut u8, len: usize) -> f64 {
    let wasm_bytes = unsafe { Vec::from_raw_parts(ptr, len, len) };
    match load_and_run_main(&wasm_bytes) {
        Ok(result) => result,
        Err(e) => {
            log(&e);
            panic!("fail");
        }
    }
}
