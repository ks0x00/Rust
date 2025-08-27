use std::os::raw::{c_int, c_double};

#[unsafe(no_mangle)]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn multiply(a: c_double, b: c_double) -> c_double {
    a * b
}

// Cargo.toml 추가
// [lib]
// crate-type = ["cdylib"]   # C용 동적 라이브러리

// build
// cargo build
// 또는
// cargo build --release

// 생성된 파일
// demo710_dll_export.dll, demo710_dll_export.dll.lib, 등

// To see how to import dll, see demo711-dll-import

// C 실행 코드
// #include <stdio.h>

// // Rust DLL 함수 선언 (헤더처럼 작성)
// __declspec(dllimport) int add(int a, int b);
// __declspec(dllimport) double multiply(double a, double b);

// int main() {
//     printf("2 + 3 = %d\n", add(2, 3));
//     printf("2.5 * 4.0 = %f\n", multiply(2.5, 4.0));
//     return 0;
// }