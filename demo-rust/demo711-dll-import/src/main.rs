use std::os::raw::{c_int, c_double};

// #[link(name = "C:/Users/Ohyoung Kwon/Documents/kpark/rust/demo-rust/target/release/demo410_dll_export.dll", kind = "dylib")]
#[link(name = "target/release/demo410_dll_export.dll", kind = "dylib")]
unsafe extern "C" {
	pub fn add(a: c_int, b: c_int) -> c_int;
	pub fn multiply(a: c_double, b: c_double) -> c_double;
}

fn main() {
	unsafe {
		println!("2 + 3 = {}", add(2, 3));
		println!("2.5 * 4.0 = {}", multiply(2.5, 4.0));
	}
}
/*
타입 매핑 주의사항
	Rust 타입       C 타입 (Windows 64비트)	      권장 이유
	c_int           int	                        정수 (32bit)
	c_long          long (플랫폼 의존)	          가급적 c_int 권장
	c_double        double                      실수
	*const c_char   const char*                 문자열 입력
	*mut c_char	    char*	                    문자열 출력/수정
	*mut c_void	    void*	                    포인터 일반

Rust 전용 타입(String, Vec<T>, Result, Option)은 FFI에 직접 노출하면 안 되고, 포인터 기반으로 변환하거나 별도 래퍼 작성 필요.
*/