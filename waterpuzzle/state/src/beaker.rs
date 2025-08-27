use std::fmt;

use crate::{MAX_WATERS, Water};

pub const NO_WATER: Water = 0;
pub(crate) const EMPTY: u32 = NO_WATER | (NO_WATER << 8) | (NO_WATER << 16) | (NO_WATER << 24);

#[derive(Clone, Copy, Debug, Eq)]
pub struct Beaker {
    waters: u32,
    pub n_waters: usize,
    pub h: u32,
}

impl Beaker {
    pub(crate) fn new() -> Self {
        Self {
            waters: EMPTY,
            n_waters: 0,
            h: 0,
        }
    }
    fn _from_waters(waters: u32) -> Self {
        let mut w = Self {
            waters,
            n_waters: 0,
            h: 0,
        };
        w.compute_n_waters();
        w.reset_h();
        w
    }

    /// This method should only be used for initialization, such as in `State.random_generate()`. See also `Beaker.push()`.
    pub(crate) fn add_water(&mut self, water: Water) {
        self.waters |= water << (8 * self.n_waters);
        self.n_waters += 1;
        self.reset_h()
    }

    pub fn water(&self, order: usize) -> Water {
        (self.waters >> (8 * order)) & 0xFF
    }

    pub fn waters(&self) -> u32 {
        self.waters
    }

    pub(crate) fn set_waters(&mut self, waters: u32) {
        self.waters = waters;
        self.compute_n_waters();
        self.reset_h();
    }

    pub fn top_water(&self) -> Water {
        if self.n_waters == 0 {
            NO_WATER
        } else {
            (self.waters >> (8 * (self.n_waters - 1))) & 0xFF
        }
    }

    pub fn top_count(self) -> usize {
        if self.n_waters == 0 {
            return 0;
        }
        let top_water = self.top_water();
        let mut count = 0;
        for i in (0..self.n_waters).rev() {
            if self.water(i) != top_water {
                break;
            }
            count += 1;
        }
        count
    }

    pub fn n_blanks(&self) -> usize {
        MAX_WATERS - self.n_waters
    }

    pub fn empty(&mut self) {
        self.waters = EMPTY;
        self.n_waters = 0;
        self.h = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.n_waters == 0
    }

    pub fn is_full(&self) -> bool {
        self.n_waters == MAX_WATERS
    }

    pub fn is_completed(&self) -> bool {
        let w = self.water(0);
        w == self.water(1) && w == self.water(2) && w == self.water(3)
    }

    pub fn can_push(&self, water: Water) -> bool {
        self.n_waters == 0 || (self.n_waters < MAX_WATERS && self.top_water() == water)
    }

    /// See alse Beaker.add_water()
    pub(crate) fn push(&mut self, water: Water, count: usize) -> bool {
        // assert!(0 < water && 0 < count && count <= MAX_WATERS);
        if self.n_waters + count > MAX_WATERS {
            return false;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////
        // bug check
        // let copy = self.clone();
        ////////////////////////////////////////////////////////////////////////////////////////////////
        if self.n_waters == 0 {
            self.h = 1;
        }
        for _ in 0..count {
            self.waters |= water << (8 * self.n_waters);
            self.n_waters += 1;
        }
        if self.n_waters == MAX_WATERS {
            self.h -= 1;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////
        // bug check
        // let ch = _compute_h(self.waters);
        // if ch != self.h {
        //     println!(  "{}", copy);
        //     println!(  "{} {}", ch, self.h);
        //     println!(  "{} {}", water, count);
        // }
        // assert!(_compute_h(self.waters) == self.h);
        ////////////////////////////////////////////////////////////////////////////////////////////////
        true
    }

    /// 스택의 위에서부터 최대 max_count 개의 Water를 뽑아낸다. 뽑아내는 Water의 색은 top의 색과 같아야 한다.
    /// max_count: 뽑아내는 Water의 최대 개수
    /// return: 뽑아낸 water(== self.top), 뽑아낸 개수
    pub(crate) fn pop(&mut self, max_count: usize) -> (Water, usize) {
        if self.n_waters == 0 {
            return (NO_WATER, 0);
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////
        // bug check
        // let copy = self.clone();
        ////////////////////////////////////////////////////////////////////////////////////////////////
        if self.n_waters < MAX_WATERS {
            self.h -= 1;
        }
        let top_water = self.top_water();
        let mut curr_water = top_water;
        let mut count = 0;
        while count < max_count && top_water == curr_water {
            count += 1;
            self.n_waters -= 1;
            self.waters &= !(0xFF << (8 * self.n_waters));
            if self.n_waters == 0 {
                curr_water = NO_WATER;
                break;
            }
            curr_water = self.top_water();
        }
        if top_water == curr_water {
            self.h += 1;
        }
        ////////////////////////////////////////////////////////////////////////////////////////////////
        // bug check
        // let ch = _compute_h(self.waters);
        // if ch != self.h {
        //     println!(  "{}", copy);
        //     println!(  "{} {}", ch, self.h);
        //     println!(  "{} {} {}", curr_water, top_water, count);
        // }
        // assert!(_compute_h(self.waters) == self.h);
        ////////////////////////////////////////////////////////////////////////////////////////////////
        (top_water, count)
    }

    /// self의 water를 beaker에 부을 수 있고, 붓고 난 다음 self의 top이 부은 색이 아니면 true, 그렇지 않으면 false
    pub fn pour_results_different_state(&self, beaker: &Self) -> bool {
        self.can_pour(beaker) && self.top_count() <= beaker.n_blanks()
    }

    pub fn can_pour(&self, beaker: &Self) -> bool {
        self.n_waters > 0 && beaker.can_push(self.top_water())
    }

    pub(crate) fn pour(&mut self, beaker: &mut Self) -> bool {
        if self.can_pour(beaker) {
            // println!("can pour");
            let water_count_pair = self.pop(beaker.n_blanks());
            // println!("{:?}", water_count_pair);
            beaker.push(water_count_pair.0, water_count_pair.1)
        } else {
            false
        }
    }

    fn compute_n_waters(&mut self) {
        for i in 0..MAX_WATERS {
            if ((self.waters >> (i << 3)) & 0xFF) == NO_WATER {
                self.n_waters = i;
                return;
            }
        }
        self.n_waters = MAX_WATERS
    }

    pub fn _h(&self) -> u32 {
        self.h
    }

    pub(crate) fn reset_h(&mut self) {
        self.h = 0;
        let water0 = (self.waters >> (0 * 8)) & 0xFF;
        let water1 = (self.waters >> (1 * 8)) & 0xFF;
        if water0 != water1 {
            self.h += 1;
        }
        let water2 = (self.waters >> (2 * 8)) & 0xFF;
        if water1 != water2 {
            self.h += 1;
        }
        let water3 = (self.waters >> (3 * 8)) & 0xFF;
        if water2 != water3 {
            self.h += 1;
        }
    }
}

impl PartialEq for Beaker {
    fn eq(&self, other: &Self) -> bool {
        self.waters == other.waters
    }
}

impl fmt::Display for Beaker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let water0 = (self.waters >> (0 * 8)) & 0xFF;
        let water1 = (self.waters >> (1 * 8)) & 0xFF;
        let water2 = (self.waters >> (2 * 8)) & 0xFF;
        let water3 = (self.waters >> (3 * 8)) & 0xFF;
        write!(
            f,
            "({} {} {} {}; {})",
            water3, water2, water1, water0, self.h
        )
    }
}

// only for assert!()
fn _compute_h(waters: u32) -> u32 {
    let mut h = 0;
    let water0 = (waters >> (0 * 8)) & 0xFF;
    let water1 = (waters >> (1 * 8)) & 0xFF;
    if water0 != water1 {
        h += 1;
    }
    let water2 = (waters >> (2 * 8)) & 0xFF;
    if water1 != water2 {
        h += 1;
    }
    let water3 = (waters >> (3 * 8)) & 0xFF;
    if water2 != water3 {
        h += 1;
    }
    h
}
