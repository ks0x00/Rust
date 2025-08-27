use std::{sync::Arc, thread};

// Arc는 다중 스레드 환경에 적합하다.
// 값을 변경하려면 Mutex와 결합한다.
fn main() {
    let a = Arc::new(1);
    let a1 = a.clone();
    let th1 = thread::spawn(move || {
        println!("th1: {}", a1);
    });
    let th2 = thread::spawn(move || {
        let a2 = a.clone();
        let th3 = thread::spawn(move || {
            println!("th3: {}", a2);
        });
        th3.join().unwrap();
        println!("th2: {}", a);
    });
    th1.join().unwrap();
    th2.join().unwrap();
}

// Rc는 단일 스레드 환경에 적합하다.
// 대신 가볍고 빠르다.
// mod rc_error {
//     use std::{rc::Rc, thread};

//     fn f() {
//         let a = Rc::new(1);
//         // 다른 스레드로 이동은 불가.
//         thread::spawn(move || {
//             println!("{}", a);
//         });
//     }
// }
