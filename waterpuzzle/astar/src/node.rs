use std::rc::Rc;

use state::state::State;

#[derive(Clone, Debug, Eq)]
pub struct Node {
    pub state: Rc<State>,
    pub g: u32,
    f: u32,
}

impl Node {
    pub(crate) fn new(state: Rc<State>, g: u32) -> Self {
        let f = g + state.h;
        Self { state, g, f }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

// BinaryHeap이 Node를 비교할 수 있도록 Ord 트레이트 구현
// f_cost가 가장 작은 노드가 "가장 작다"고 간주되어 pop될 수 있도록 합니다.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // f_cost를 기준으로 비교하되, 최소 힙처럼 동작하도록 역순으로 비교합니다.
        // 즉, f_cost가 작은 것이 더 "크다"고 간주되어 BinaryHeap에서 먼저 pop되도록 합니다.
        // 또는 그냥 self.f_cost.cmp(&other.f_cost).reverse() 를 사용할 수도 있습니다.
        // my codes
        // let cmp = other.f.cmp(&self.f);
        // if cmp == std::cmp::Ordering::Equal {
        //     return self.g.cmp(&other.g);
        // }
        // cmp
        other.f.cmp(&self.f).then_with(|| self.g.cmp(&other.g)) // f가 같으면 g가 작은 것을 우선
        // f_cost가 같을 경우, g_cost가 더 큰 것을 우선시할 수 있습니다 (tie-breaking heuristic)
        // .then_comparing(&self.g_cost) // 이 부분은 선택 사항
        // 만약 State가 Eq, PartialEq만 구현하고 Ord, PartialOrd를 구현하지 않았다면,
        // 이 Node의 cmp 구현에서는 state 필드를 직접 비교할 수 없습니다.
        // 하지만 A*에서는 f_cost만으로도 충분합니다.
    }
}

// PartialOrd도 구현해야 합니다.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
