use std::rc::Rc;

// Rc - reference counted
// Rc는 단일 스레드 용.
// https://doc.rust-kr.org/ch16-04-extensible-concurrency-sync-and-send.html
// Rc<T> 값을 복제하여 다른 스레드로 복제본의 소유권 전송을 시도한다면,
// 두 스레드 모두 동시에 참조 카운트 값을 업데이트할지도 모르기 때문입니다.
// 이러한 이유로, Rc<T>는 스레드-안전성 성능 저하를 지불하지 않아도 되는 싱글스레드의 경우에 사용되도록 구현되었습니다.
fn main() { 
    let a = Rc::new("big data");
    println!("{a}, {}", Rc::strong_count(&a));

    let b = a.clone(); // 주소 복사
    println!("{b}, {}", Rc::strong_count(&b));
}
