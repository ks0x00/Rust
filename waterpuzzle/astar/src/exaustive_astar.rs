use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
    sync::mpsc,
};

use state::state::State;

use crate::node::Node;

#[derive(Debug, Default)]
pub struct ExaustiveAStar {
    open_set: BinaryHeap<Node>,
    // came_from과 g_score 맵은 이제 Rc<State>를 키로 사용
    came_from: HashMap<Rc<State>, Rc<State>>,
    g_score: HashMap<Rc<State>, u32>,
    goal: Option<Rc<State>>,
    pub message: String,
}

impl ExaustiveAStar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn find_path(
        &mut self,
        initial_state: State,
        stop_rx: mpsc::Receiver<()>,
    ) -> Option<Vec<State>> {
        let initial_arc_state = Rc::new(initial_state); // 초기 상태를 Arc로 감쌈

        // Arc의 clone()을 호출하면 참조 카운터만 증가한다.
        self.open_set.push(Node::new(initial_arc_state.clone(), 0));
        self.g_score.insert(initial_arc_state.clone(), 0);

        let mut epoch = 0;
        while let Some(current_node) = self.open_set.pop() {
            if stop_rx.try_recv().is_ok() {
                self.message = "Stopped".to_string();
                return None;
            }
            let current_arc_state = current_node.state;

            if current_arc_state.is_completed() {
                match self.goal.as_ref() {
                    Some(goal) => {
                        let g_goal = self.g_score.get(goal).unwrap();
                        let g_current = self
                            .g_score
                            .get(&current_arc_state)
                            .expect("State has no g value");
                        if g_current < g_goal {
                            self.goal = Some(current_arc_state);
                        }
                    }
                    None => self.goal = Some(current_arc_state),
                }
                continue;
            }

            // A*의 g_score 검사 (중복 처리)
            if let Some(&existing_g) = self.g_score.get(&current_arc_state) {
                if current_node.g > existing_g {
                    continue;
                }
            }

            // NextStateIterator를 사용하거나 직접 루프를 돌 수 있습니다.
            // 여기서는 NextStateIterator를 가정합니다. (State::new()의 capacities 인자 때문에 약간의 수정 필요)
            // NextStateIterator는 &State를 받으므로, Rc::deref()를 사용하여 &State로 변환 가능
            // for neighbor_state in NextStateIterator::new(&current_arc_state) {
            // ...
            // }

            epoch += 1;
            let next_g = current_node.g + 1;

            // 직접 루프 사용
            for i in 0..current_arc_state.beakers.len() {
                for j in 0..current_arc_state.beakers.len() {
                    if i == j {
                        continue;
                    }

                    if current_arc_state.can_pour(i, j) {
                        // Rc::make_mut()는 참조 카운트가 1이 아니면 복사본을 생성합니다.
                        // 하지만 여기서는 새로운 상태를 만들어야 하므로 그냥 clone()을 호출하고 pour 합니다.
                        // Rc<State>를 Clone()하면 참조 카운트만 증가합니다.
                        // 실제 상태를 변경하려면 Rc 내부의 State를 mutable하게 만들어야 합니다.
                        // 따라서, 새로운 상태를 만들 때는 Rc::clone()이 아니라 State::clone()을 해야 합니다.
                        // current_arc_state는 Rc<State>이므로 *current_arc_state는 &State 입니다.
                        // 따라서 (*current_arc_state).clone()으로 State 자체를 복사합니다.

                        let mut neighbor_state = (*current_arc_state).clone(); // State를 깊은 복사
                        neighbor_state.pour(i, j); // 복사된 State 변경

                        let neighbor_arc_state = Rc::new(neighbor_state); // 변경된 State를 다시 Arc로 감쌈

                        if next_g
                            < *self
                                .g_score
                                .get(&neighbor_arc_state)
                                .unwrap_or(&std::u32::MAX)
                        {
                            self.g_score.insert(neighbor_arc_state.clone(), next_g);
                            self.came_from
                                .insert(neighbor_arc_state.clone(), current_arc_state.clone());

                            let neighbor_node = Node::new(neighbor_arc_state.clone(), next_g);
                            self.open_set.push(neighbor_node);
                        }
                    }
                }
            }
        }
        match self.goal.as_ref() {
            Some(goal) => {
                let path = self.construct_path_from(goal.clone());
                self.message = format!(
                    "Path length: {}, Epoch: {epoch}, Size of g_score: {}",
                    path.len() - 1,
                    self.g_score.len()
                );
                Some(path)
            }
            None => {
                self.message = format!(
                    "Fail to find a path, Epoch: {epoch}, Size of g_score: {}",
                    self.g_score.len()
                );
                None
            }
        }
    }

    fn construct_path_from(&self, mut arc_state: Rc<State>) -> Vec<State> {
        let mut path = vec![(*arc_state).clone()]; // Rc<State>에서 State를 복사
        while self.came_from.contains_key(&arc_state) {
            arc_state = self.came_from.get(&arc_state).unwrap().clone();
            path.push((*arc_state).clone());
        }
        path.reverse();
        path
    }
}
