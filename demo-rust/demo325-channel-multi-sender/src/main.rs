use std::{io::Write, sync::mpsc, thread};

type DataSender = mpsc::Sender<i32>;
// type DataReceiver = mpsc::Receiver<i32>;

fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..3 {
        let tx = tx.clone();
        sender(tx, i * 10);
    }
    drop(tx);

    loop {
        match rx.recv() {
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

fn sender(tx: DataSender, start: i32) {
    thread::spawn(move || {
        for n in 0..5 {
            let _ = tx.send(start + n);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}
