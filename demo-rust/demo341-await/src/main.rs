// async-await를 사용하기 위한 런타임이 필요합니다.
// Cargo.toml에 다음을 추가 (예: Tokio 런타임)
// [dependencies]
// tokio = { version = "1", features = ["full"] }

use tokio::time::{sleep, Duration};

// async 키워드는 이 함수가 비동기적으로 실행될 수 있음을 나타냅니다.
// 이 함수는 Future를 반환합니다.
async fn make_coffee(order_num: u32) -> String {
    println!("{}번 손님의 커피 만드는 중...", order_num);
    // await 키워드는 이 지점에서 작업이 완료될 때까지 "기다리지만",
    // 그 동안 다른 작업을 할 수 있도록 제어권을 넘겨줍니다.
    sleep(Duration::from_secs(2)).await; // 2초 동안 커피를 만듭니다.
    format!("{}번 손님의 커피 완성!", order_num)
}

// main 함수도 비동기여야 합니다. (tokio::main 매크로 사용)
#[tokio::main]
async fn main() {
    println!("커피 주문 받기 시작!");

    // 여러 커피를 동시에 만들기 시작합니다.
    // 각 make_coffee 호출은 별도의 Future를 반환합니다.
    let coffee1_future = make_coffee(1);
    let coffee2_future = make_coffee(2);

    // await는 해당 Future가 완료될 때까지 기다립니다.
    // 하지만 이 기다림은 블로킹이 아닙니다.
    // await coffee1_future; 가 실행되는 동안에도
    // 다른 비동기 작업(예: coffee2_future)은 계속 진행될 수 있습니다.

    let result1 = coffee1_future.await; // 1번 커피가 완성될 때까지 기다림
    println!("{}", result1);

    let result2 = coffee2_future.await; // 2번 커피가 완성될 때까지 기다림
    println!("{}", result2);

    println!("모든 커피 주문 처리 완료!");
}