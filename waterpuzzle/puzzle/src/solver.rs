use astar::Astar;
use state::INFINITY_USIZE;
use state::state::State;
use std::sync::mpsc; // Multiple Producer, Single Consumer 채널
use std::{thread, usize};

// A* 결과의 수신자 (리시버) 타입을 정의합니다.
// Vec<State>는 A* 탐색 결과 경로입니다. None은 경로를 찾지 못했음을 의미합니다.
type DataReceiver = mpsc::Receiver<Option<Vec<State>>>;
type StopSender = mpsc::Sender<()>; // 스레드에 종료 신호를 보낼 Sender

#[derive(Debug, Default)]
pub struct Solver {
    data_receiver: Option<DataReceiver>,
    stop_sender: Option<StopSender>,
    pub path: Option<Option<Vec<State>>>,
}

impl Solver {
    pub fn remaining_step(&self) -> Option<usize> {
        self.path.as_ref().map(|path| match path {
            Some(path) => path.len() - 1,
            None => INFINITY_USIZE,
        })
    }

    pub fn hint(&self) -> Option<&State> {
        if let Some(path) = self.path.as_ref() {
            if let Some(path) = path {
                if path.len() > 1 {
                    return Some(&path[1]);
                }
            }
        }
        None
    }

    // solve 함수는 이제 A* 탐색을 시작하고, Receiver를 Solver 내부에 저장합니다.
    pub fn solve(&mut self, initial_state: &State) {
        self.stop();
        self.path = None;

        let (data_tx, data_rx) = mpsc::channel(); // Sender(data_tx)와 Receiver(data_rx)를 생성합니다.
        let (stop_tx, stop_rx) = mpsc::channel();

        let state_for_thread = initial_state.clone(); // 초기 상태를 클론하여 스레드로 이동

        thread::spawn(move || {
            let mut astar = Astar::new();
            let found_path = astar.find_path(state_for_thread, stop_rx); // A* 탐색 수행

            data_tx.send(found_path).expect("Failed to send A* path");
        });

        // 생성된 Sender와 Receiver를 Solver 내부에 저장합니다.
        self.data_receiver = Some(data_rx);
        self.stop_sender = Some(stop_tx);
    }

    // A* 스레드로부터 결과가 도착했는지 확인하고, 도착했다면 Solver 내부에 설정합니다.
    // 결과가 설정되었으면 true를 반환하고, 아직 도착하지 않았거나Receiver가 없으면 false를 반환합니다.
    pub fn check(&mut self) {
        if let Some(receiver) = &self.data_receiver {
            match receiver.try_recv() {
                Ok(data) => {
                    self.path = Some(data);
                    self.data_receiver = None; // 더 이상 이 Receiver는 유효하지 않으므로 제거
                    self.stop_sender = None;
                }
                Err(mpsc::TryRecvError::Empty) => {} // 아직 메시지가 도착하지 않음
                Err(mpsc::TryRecvError::Disconnected) => {
                    // Sender가 끊어진 경우 (스레드가 종료되었으나 메시지를 보내지 못한 경우 등)
                    self.path = None;
                    self.data_receiver = None; // 더 이상 이 Receiver는 유효하지 않으므로 제거
                    self.stop_sender = None;
                }
            }
        }
    }

    pub fn stop(&mut self) {
        if let Some(sender) = self.stop_sender.take() {
            // Sender를 드롭하면 Receiver에서 Disconnected 에러가 발생하지만,
            // 명시적으로 `send(())`를 하는 것이 의도를 명확히 합니다.
            // 에러를 무시하는 것은 Receiver가 이미 끊어졌을 수 있기 때문입니다.
            let _ = sender.send(());
        }
    }
}
