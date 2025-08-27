use std::{thread, time::Duration};

fn main() {
    func1();
    let _ = func2();
}

fn func1() {
    thread::spawn(move || {
        println!("func1");
    });
}

fn func2() -> Result<(), Box<dyn std::any::Any + Send + 'static>> {
    let th = thread::spawn(move || {
        println!("start func2");
        thread::sleep(Duration::from_millis(1000));
        println!("end func2");
    });
    th.join()
}
