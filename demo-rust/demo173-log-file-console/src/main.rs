use chrono::Local;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use std::{fs::OpenOptions, io::Write};

pub fn start_log() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open("error.log")
        .unwrap();
    let header = format!(
        "---- {} ----\r\n",
        Local::now().format("%Y-%m-%d %H::%M::%S")
    );
    file.write_all(header.as_bytes()).unwrap();
    CombinedLogger::init(vec![
        // 콘솔 출력 (Info 이상)
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // 파일 출력 (Error 이상)
        WriteLogger::new(
            LevelFilter::Error,
            Config::default(),
            // File::create("error.log").unwrap(),
            file,
        ),
    ])
    .unwrap();
}

fn main() {
    start_log();
    // 로그 테스트
    log::trace!("trace 메시지 - 무시됨");
    log::debug!("debug 메시지 - 무시됨");
    log::info!("info 메시지 - 콘솔에만 출력");
    log::warn!("warn 메시지 - 콘솔에만 출력");
    log::error!("error {} - 콘솔 + 파일 출력", "메시지");

    test_function();
}

fn test_function() {
    log::error!("함수 내부에서 발생한 오류");
}
