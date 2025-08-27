/*
log level - info, warn, error, debug, trace

> cargo add log@0.4
> cargo add env_logger@0.10

Cargo.toml
    [dependencies]
    log = "0.4"
    env_logger = "0.10"
 
실행
    error 레벨 로그만 출력
    Bash
        > RUST_LOG=error cargo run
    PowerShell
        > $env:RUST_LOG="error"; cargo run
    CMD
        > set RUST_LOG=error && cargo run

    모든 레벨 로그 출력
        > RUST_LOG=trace cargo run
    
레벨 순서
    trace > debug > info > warn > error
*/

pub trait ParseAs {
    fn parse_as<T>(&self, who: &str) -> T
    where
        T: std::str::FromStr + std::default::Default,
        <T as std::str::FromStr>::Err: std::fmt::Debug;
}

impl ParseAs for &str {
    fn parse_as<T>(&self, who: &str) -> T
    where
        T: std::str::FromStr + std::default::Default,
        <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        log::info!("{who} - '{self}' 파싱 시작");
        log::trace!("{who} - '{self}' 파싱 시작");
        log::debug!("{who} - '{self}' 파싱 시작");
        self.trim().parse::<T>().unwrap_or_else(|e| {
            if !self.is_empty() {
                // eprintln! 대신 error! 매크로를 사용
                // 에러 로그는 기본적으로 stderr로 출력됩니다.
                log::error!("{who} - '{self}' 파싱 실패: {e:?}");
                log::warn!("{who} - '{self}' 파싱 실패: {e:?}");
            }
            T::default()
        })
    }
}

fn main() {
    // env_logger를 초기화
    // 이 코드는 프로그램 전체에서 한 번만 호출하면 됩니다.
    env_logger::init();
    
    // 사용 예시 (이전과 동일)
    let s_u64 = "  123   ";
    let s_invalid = "abc";
    
    let num_u64: u64 = s_u64.parse_as("main() - s_64");
    println!("u64: {}", num_u64); 

    let invalid_num: u64 = s_invalid.parse_as("main() - s_invalid");
    println!("Invalid num (default): {}", invalid_num); 
}
