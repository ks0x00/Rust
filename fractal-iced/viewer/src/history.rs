pub struct History {
    list: Vec<String>,
    curr: usize,
}

impl History {
    pub fn init(text: String) -> Self {
        let list = vec![text];
        Self { curr: 0, list }
    }

    pub fn current(&self) -> &str {
        return &self.list[self.curr];
    }

    pub fn undo(&mut self) -> Option<&str> {
        if self.curr > 0 {
            self.curr -= 1;
            return Some(&self.list[self.curr]);
        } else {
            return None;
        }
    }

    pub fn redo(&mut self) -> Option<&str> {
        if self.curr + 1 < self.list.len() {
            self.curr += 1;
            return Some(&self.list[self.curr]);
        } else {
            return None;
        }
    }

    pub fn push(&mut self, text: String) {
        if self.list[self.curr] != text {
            self.list.truncate(self.curr + 1);
            self.list.push(text);
            self.curr += 1;
        }
    }
}
