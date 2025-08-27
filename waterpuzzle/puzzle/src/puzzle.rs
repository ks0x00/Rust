use std::{
    error::Error,
    io::{Read, Write},
};

use state::{Cord, INFINITY_USIZE, beaker::Beaker, state::State};

use crate::{history::History, solver::Solver};

#[derive(Debug, Default)]
pub struct Puzzle {
    state: State,
    selected_beaker_index: Option<usize>,
    // hint_src_beaker_index: Option<usize>,
    hint_dst_beaker_index: Option<usize>,
    history: History,
    solver: Solver,
}

impl Puzzle {
    pub fn empty_new(n_beakers: usize) -> Self {
        let state = State::new(n_beakers);
        let history = History::new(state.cord());
        Self {
            state,
            history,
            ..Default::default()
        }
    }

    pub fn random_new(n_beakers: usize) -> Self {
        let mut state = State::new(n_beakers);
        state.random_generate();
        let history = History::new(state.cord());
        let mut puzzle = Self {
            state,
            history,
            ..Default::default()
        };
        puzzle.solve();
        puzzle
    }

    pub fn reset(&mut self) {
        self.state.random_generate();
        self.selected_beaker_index = None;
        self.history.reset(self.state.cord());
        self.solve();
    }

    pub fn n_beakers(&self) -> usize {
        self.state.n_beakers()
    }

    pub fn selected_beaker(&self) -> Option<&Beaker> {
        match self.selected_beaker_index {
            Some(index) => Some(self.state.beaker(index)),
            None => None,
        }
    }

    pub fn is_selected_beaker_index(&self, index: usize) -> bool {
        Some(index) == self.selected_beaker_index
    }

    // pub fn is_hint_src_beaker_index(&self, index: usize) -> bool {
    //     Some(index) == self.hint_src_beaker_index
    // }
    pub fn is_hint_dst_beaker_index(&self, index: usize) -> bool {
        Some(index) == self.hint_dst_beaker_index
    }

    pub fn beaker(&self, index: usize) -> &Beaker {
        self.state.beaker(index)
    }

    pub fn step(&self) -> usize {
        self.history.step()
    }

    pub fn is_completed(&self) -> bool {
        self.state.is_completed()
    }

    pub fn pouring_results_different_state(&self) -> bool {
        self.state.pour_results_different_state()
    }

    pub fn on_right_click(&mut self, clicked_beaker_index: Option<usize>) {
        // 비커가 아닌 빈 공간을 클릭했으면 선택을 해제한다.
        let Some(clicked_index) = clicked_beaker_index else {
            self.selected_beaker_index = None;
            return; // 함수 종료
        };

        // 특정 비커를 클릭한 경우
        self.hint_dst_beaker_index = None;
        if let Some(selected_index) = self.selected_beaker_index {
            // 이미 선택된 비커가 있는 경우
            if clicked_index == selected_index {
                // 선택된 비커를 다시 클릭하면 선택 해제
                self.selected_beaker_index = None;
            } else {
                // 선택된 비커에서 클릭한 비커로 물을 붓는다.
                if self.state.pour(selected_index, clicked_index) {
                    // 성공적으로 부었으면 현재 상태를 history에 저장한다.
                    self.history.push(self.state.cord());
                    self.solve();
                }
                // 붓기 성공/실패와 관계없이 선택을 해제한다.
                self.selected_beaker_index = None;
            }
        } else {
            // 선택된 비커가 없는 경우
            // 클릭한 비커가 비어 있지 않으면 선택한다.
            if !self.state.beaker(clicked_index).is_empty() {
                self.selected_beaker_index = Some(clicked_index);
            }
        }
    }

    pub fn on_left_click(&mut self, clicked_beaker_index: usize) {
        self.selected_beaker_index = None;
        self.hint_dst_beaker_index = None;
        if let Some(target) = self.state.pourable_beaker_index(clicked_beaker_index) {
            self.state.pour(clicked_beaker_index, target);
            self.history.push(self.state.cord());
            self.solve();
        }
    }

    pub fn undo(&mut self) {
        if let Some(cord) = self.history.undo() {
            self.state.apply_cord(cord);
            self.solve();
        }
    }

    pub fn undo_all(&mut self) {
        self.state.apply_cord(self.history.undo_all());
        self.solve();
    }

    pub fn redo(&mut self) {
        if let Some(cord) = self.history.redo() {
            self.state.apply_cord(cord);
            self.solve();
        }
    }

    pub fn solve(&mut self) {
        // self.hint_src_beaker_index = None;
        self.selected_beaker_index = None;
        self.hint_dst_beaker_index = None;
        self.solver.solve(&self.state);
    }

    pub fn check_solver(&mut self) {
        self.solver.check();
    }

    pub fn reset_solver_hint(&mut self) {
        if let Some(hint_state) = self.solver.hint() {
            self.selected_beaker_index = None;
            for (i, hint_beaker) in hint_state.beakers.iter().enumerate() {
                let beaker = self.state.beaker(i);
                if hint_beaker.n_waters < beaker.n_waters {
                    // self.hint_src_beaker_index = Some(i);
                    self.selected_beaker_index = Some(i);
                } else if hint_beaker.n_waters > beaker.n_waters {
                    self.hint_dst_beaker_index = Some(i);
                }
            }
        }
    }

    pub fn apply_solver_hint(&mut self) {
        if let Some(next_state) = self.solver.hint() {
            self.state = next_state.clone();
            self.history.push(self.state.cord());
            self.selected_beaker_index = None;
            self.solve();
        }
    }

    pub fn solver_emoticon(&self) -> String {
        match self.solver.remaining_step() {
            Some(step) => {
                if step == INFINITY_USIZE {
                    ":-(".to_owned()
                } else {
                    ":-)".to_owned()
                }
            }
            None => "".to_owned(),
        }
    }

    pub fn solver_remaining_step(&self) -> String {
        match self.solver.remaining_step() {
            Some(step) => {
                if step == INFINITY_USIZE {
                    "\u{221e}".to_owned()
                } else {
                    step.to_string()
                }
            }
            None => "".to_owned(),
        }
    }

    pub fn save<W: Write>(&self, bw: &mut W) -> Result<(), Box<dyn Error>> {
        for x in self.state.cord() {
            bw.write_all(&x.to_le_bytes())?;
        }
        self.history.save(bw)?;
        Ok(())
    }

    pub fn load<R: Read>(&mut self, br: &mut R) -> Result<(), Box<dyn Error>> {
        let n_beakers = self.n_beakers();
        let mut cord = Cord::with_capacity(n_beakers);
        let mut buf = [0; 4];
        for _ in 0..n_beakers {
            br.read_exact(&mut buf)?;
            cord.push(u32::from_le_bytes(buf));
        }
        self.state.apply_cord(&cord);
        self.history.load(br, n_beakers)?;
        self.solve();
        Ok(())
    }
}
