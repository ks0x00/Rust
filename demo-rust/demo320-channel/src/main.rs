use std::{
    io::Write,
    sync::mpsc,
    thread::{self, JoinHandle},
};

fn main() {
    func1();
    println!();
    func2();
}

fn func1() {
    // type StopSender = mpsc::Sender<()>;
    // type StopReceiver = mpsc::Receiver<()>;
    let (sender, receiver) = mpsc::channel();
    let th = thread::spawn(move || {
        let mut n = 0;
        loop {
            print!("{n} ");
            std::io::stdout().flush().unwrap();
            thread::sleep(std::time::Duration::from_millis(100));
            // 메시지를 받았는지 확인한다
            if receiver.try_recv().is_ok() {
                // 메시지 처리
                break;
            }
            n += 1;
        }
    });

    thread::sleep(std::time::Duration::from_millis(2000));
    // 메시지를 보낸다.
    let _ = sender.send(());
    let _ = th.join();
}

fn func2() {
    // type DataSender = mpsc::Sender<i32>;
    // type DataReceiver = mpsc::Receiver<i32>;
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        for n in 0..5 {
            let _ = sender.send(n);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    receive_blocking(receiver);
    // receive_blocking_another(receiver);
    // receive_nonblocking(receiver);
    // let _ = receive_in_thread(receiver).join();
}

/// 데이터가 올 때까지 기다리는 함수
#[allow(dead_code)]
fn receive_blocking(receiver: mpsc::Receiver<i32>) {
    // blocking
    loop {
        match receiver.recv() {
            // data가 올 때까지 기다린다.
            Ok(n) => {
                print!("{n } ");
                std::io::stdout().flush().unwrap();
            }
            Err(mpsc::RecvError) => {
                println!("Disconnected");
                break;
            }
        }
    }
}

/// 데이터가 올 때까지 기다리는 함수
#[allow(dead_code)]
fn receive_blocking_another(receiver: mpsc::Receiver<i32>) {
    // blocking
    for n in receiver {
        print!("{n } ");
        std::io::stdout().flush().unwrap();
    }
    println!("Disconnected");
}

/// 작업 중간에 메시지를 확인하는 방법
#[allow(dead_code)]
fn receive_nonblocking(receiver: mpsc::Receiver<i32>) {
    // non-blocking
    loop {
        match receiver.try_recv() {
            Ok(n) => {
                print!("{n} ");
                std::io::stdout().flush().unwrap();
            }
            Err(mpsc::TryRecvError::Empty) => {
                print!("Empty ");
                std::io::stdout().flush().unwrap();
            } // 아직 메시지가 도착하지 않음
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Disconnected");
                break;
            }
        }
        thread::sleep(std::time::Duration::from_millis(50));
    }
}

/// 다른 쓰레드에서 메시지 받기
#[allow(dead_code)]
fn receive_in_thread(receiver: mpsc::Receiver<i32>) -> JoinHandle<()> {
    // blocking
    thread::spawn(move || {
        loop {
            match receiver.recv() {
                // data가 올 때까지 기다린다.
                Ok(n) => {
                    print!("{n } ");
                    std::io::stdout().flush().unwrap();
                }
                Err(mpsc::RecvError) => {
                    println!("Disconnected");
                    break;
                }
            }
        }
    })
}
