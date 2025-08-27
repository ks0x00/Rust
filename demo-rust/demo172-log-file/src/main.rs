use simplelog::*;
use std::fs::File;

fn main() {
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Warn,
            // LevelFilter::Error,
            Config::default(),
            File::create("error.log").unwrap(),
        ),
    ]).unwrap();

    log::error!("프로그램에서 오류 발생!");
    log::error!("다른 함수 호출 전");
    test_function();
}

fn test_function() {
    log::error!("테스트 함수에서 오류 발생!");
    log::warn!("warning")
}

/*
레벨	 매크로	          설명
Trace	log::trace!()	디버깅의 최하위 단계. 세부 실행 흐름까지 기록 (루프 내부, 변수 값 변화 등). 보통 개발 환경에서만 사용.
Debug	log::debug!()	디버깅용 정보. 프로그램 상태와 흐름 파악에 유용. 운영 환경에서는 보통 끔.
Info	log::info!()	정상 동작 중의 일반 정보. 예: “서버가 포트 8080에서 시작됨”.
Warn	log::warn!()	경고. 오류는 아니지만 잠재적 문제. 예: “설정 파일이 없어서 기본값 사용”.
Error	log::error!()	오류. 실행은 계속될 수 있으나 명백한 실패 상황.
 */