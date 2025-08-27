use libloading::{Library, Symbol};
use std::os::raw::{c_double, c_int};

const DLL: &str = "demo410_dll_export.dll";

fn exist(symbol: &str) -> bool {
    unsafe {
        let lib = Library::new(DLL).expect("DLL load failed");
        // 심볼 유무
        lib.get::<*mut ()>(symbol.as_bytes()).is_ok()
    }
}

fn add(a: i32, b: i32) -> i32 {
    unsafe {
        let lib = Library::new(DLL).expect("DLL load failed");

        let add: Symbol<unsafe extern "C" fn(c_int, c_int) -> c_int> =
            lib.get(b"add").expect("add not found");
        add(a, b)
    }
}

fn multiply(a: f64, b: f64) -> f64 {
    unsafe {
        let lib = Library::new(DLL).expect("DLL load failed");
        let multiply: Symbol<unsafe extern "C" fn(c_double, c_double) -> c_double> =
            lib.get(b"multiply").expect("multiply not found");
        multiply(a, b)
    }
}

fn main() {
    println!("{}", exist("add"));
    println!("2 + 3 = {}", add(2, 3));
    println!("2.5 * 4.0 = {}", multiply(2.5, 4.0));
}
