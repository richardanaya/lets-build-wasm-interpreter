extern crate alloc;
use crate::alloc::string::ToString;
use alloc::vec::Vec;
use watson::*;
use webassembly::*;

#[no_mangle]
fn malloc(size: usize) -> *mut u8 {
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    let ptr = buf.as_mut_ptr();
    core::mem::forget(buf);
    ptr
}

extern "C" {
    fn _log(msg: *const u8);
}

fn log(msg: &str) {
    let mut s = msg.to_string();
    s.push_str("\0");
    unsafe { _log(s.as_ptr()) }
}

fn load_and_run_main(wasm_bytes: &[u8]) -> Result<f64, String> {
    let program = Program::parse(&wasm_bytes)?;
    let main_function = program.find_exported_function("main")?;
    let main_code = program.find_code_block(main_function.index)?;
    if main_code.code[0] == I32_CONST {
        let (num, _) = main_code.code.try_extract_i32(1)?;
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
