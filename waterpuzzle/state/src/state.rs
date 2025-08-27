use std::hash::Hash;

use rand::Rng;

use crate::{beaker::Beaker, *};

const N_EMPTY_BEAKER: usize = 2;

#[derive(Default, Clone, Debug, Eq)]
pub struct State {
    pub beakers: Vec<Beaker>,
    sorted_cord: Cord,
    pub h: u32,
}

impl State {
    pub fn new(n_beakers: usize) -> Self {
        Self::from_beakers(vec![Beaker::new(); n_beakers])
    }

    pub fn from_beakers(beakers: Vec<Beaker>) -> Self {
        let sorted_cord = Self::sorted_cord_of(&beakers);
        let h = Self::h_of(&beakers);
        Self {
            beakers,
            sorted_cord,
            h,
        }
    }

    pub fn _from_cord(cord: &Cord) -> Self {
        let mut state = Self::new(cord.len());
        state.apply_cord(cord);
        state
    }

    pub fn n_beakers(&self) -> usize {
        self.beakers.len()
    }

    pub fn n_colors(&self) -> usize {
        self.n_beakers() - N_EMPTY_BEAKER
    }

    pub fn random_generate(&mut self) {
        self.clear();
        let mut water_used = vec![0; self.n_colors() + 1];
        let mut remaining_waters: Vec<usize> = (1..=self.n_colors()).collect();
        let mut remaining_beakers: Vec<usize> = (0..self.n_colors()).collect();

        let mut rng = rand::rng();
        while !remaining_waters.is_empty() {
            let water_index = rng.random_range(0..remaining_waters.len());
            let water = remaining_waters[water_index];
            water_used[water] += 1;
            if water_used[water] == MAX_WATERS {
                remaining_waters.remove(water_index);
            }

            let beaker_index = rng.random_range(0..remaining_beakers.len());
            let beaker = &mut self.beakers[remaining_beakers[beaker_index]];
            beaker.add_water(water as Water);
            if beaker.is_full() {
                remaining_beakers.remove(beaker_index);
            }
        }
        self.sorted_cord = Self::sorted_cord_of(&self.beakers);
        self.h = Self::h_of(&self.beakers);
        //////////////////////////////////////////////////////////////////////////
        // bug
        // println!("{}", self.h);
        // self.print_beakers();
        // println!("------------------------------------------------------------");
        //////////////////////////////////////////////////////////////////////////
    }

    pub fn clear(&mut self) {
        for beaker in self.beakers.iter_mut() {
            beaker.empty()
        }
        self.h = 0;
        self.sorted_cord = Self::sorted_cord_of(&self.beakers);
    }

    pub fn is_completed(&self) -> bool {
        // self.beakers.iter().all(|beaker| beaker.is_completed())
        self.h == 0
    }

    pub fn beaker(&self, index: usize) -> &Beaker {
        &self.beakers[index]
    }

    /// 붓고 난 다음 top의 색이 달라지는 비커가 있으면 true, 그렇지 않으면 false
    pub fn pour_results_different_state(&self) -> bool {
        self.beakers.iter().any(|src_beaker| {
            self.beakers.iter().any(|dst_beaker| {
                !std::ptr::eq(src_beaker, dst_beaker)
                    && src_beaker.pour_results_different_state(dst_beaker)
            })
        })
    }

    pub fn can_pour(&self, i: usize, j: usize) -> bool {
        self.beakers[i].can_pour(&self.beakers[j])
    }

    pub fn print_beakers(&self) {
        println!("number of beakers {}", self.beakers.len());
        for (i, beaker) in self.beakers.iter().enumerate() {
            println!("{i}: {beaker}");
        }
    }

    pub fn pour(&mut self, i: usize, j: usize) -> bool {
        ///////////////////////////////////////////////////////////////////////////////////////////
        // bug
        //     println!("----- {} {}", self.h, Self::h_of(&self.beakers));
        // self.print_beakers();
        ///////////////////////////////////////////////////////////////////////////////////////////
        let prev_h_at_ij = self.beakers[i].h + self.beakers[j].h;
        let poured = if i < j {
            ///////////////////////////////////////////////////////////////////////////////////////////
            // bug
            // if self.h < self.beakers[i].h + self.beakers[j].h {
            //     println!("{}, {}, {}", self.h, self.beakers[i], self.beakers[j]);
            //     self.print_beakers();
            //     println!("------------------------------------------------------------");
            // }
            ///////////////////////////////////////////////////////////////////////////////////////////
            let (src, dst) = self.beakers.split_at_mut(j);
            src[i].pour(&mut dst[0])
        } else {
            ///////////////////////////////////////////////////////////////////////////////////////////
            // bug
            // if self.h < self.beakers[i].h + self.beakers[j].h {
            //     println!("{}, {}, {}", self.h, self.beakers[i], self.beakers[j]);
            //     self.print_beakers();
            //     println!("------------------------------------------------------------");
            // }
            ///////////////////////////////////////////////////////////////////////////////////////////
            let (dst, src) = self.beakers.split_at_mut(i);
            src[0].pour(&mut dst[j])
        };
        if poured {
            self.h -= prev_h_at_ij;
            self.h += self.beakers[i].h + self.beakers[j].h;
            self.sorted_cord = Self::sorted_cord_of(&self.beakers);
            //////////////////////////////////////////////////////////////////////////
            // bug
            // if self.h != Self::h_of(&self.beakers) {
            //     println!("***** {} {}", self.h, Self::h_of(&self.beakers));
            //     self.print_beakers();
            //     println!("------------------------------------------------------------");
            // }
            // assert!(self.h == Self::h_of(&self.beakers));
            //////////////////////////////////////////////////////////////////////////
        }
        ///////////////////////////////////////////////////////////////////////////////////////////
        // bug
        //     println!("===== {} {}", self.h, Self::h_of(&self.beakers));
        // self.print_beakers();
        ///////////////////////////////////////////////////////////////////////////////////////////
        poured
    }

    pub fn pourable_beaker_index(&mut self, i: usize) -> Option<usize> {
        let mut h = UNDEFINED_U32;
        // split은 세 값 중 하나를을 가진다.
        // false: 부은 후 같은 색이 남지 않는다. 이것을 가장 선호한다.
        // true: 부은 후 같은 색이 남는다.
        // None: 아직 정해지지 않았다.
        let mut split: Option<bool> = None;
        let top_count = self.beaker(i).top_count();
        let mut target: Option<usize> = None;
        for j in 0..self.beakers.len() {
            if i == j {
                continue;
            }
            if self.can_pour(i, j) {
                match split {
                    Some(false) => {
                        if top_count <= self.beaker(j).n_blanks() && self.beaker(j).h > h {
                            h = self.beaker(j).h;
                            target = Some(j);
                        }
                    }
                    Some(true) => {
                        split = Some(top_count > self.beaker(j).n_blanks());
                        if !split.unwrap() || self.beaker(j).h > h {
                            h = self.beaker(j).h;
                            target = Some(j);
                        }
                    }
                    None => {
                        h = self.beaker(j).h;
                        target = Some(j);
                        split = Some(top_count > self.beaker(j).n_blanks());
                    }
                }
            }
        }
        target
    }

    pub fn cord(&self) -> Cord {
        self.beakers.iter().map(|beaker| beaker.waters()).collect()
    }

    fn sorted_cord_of(beakers: &[Beaker]) -> Cord {
        let mut cord: Cord = beakers.iter().map(|beaker| beaker.waters()).collect();
        cord.sort();
        cord
    }

    pub fn apply_cord(&mut self, cord: &Cord) {
        for (waters, beaker) in cord.iter().zip(self.beakers.iter_mut()) {
            beaker.set_waters(*waters)
        }
        self.sorted_cord = Self::sorted_cord_of(&self.beakers);
        self.h = Self::h_of(&self.beakers);
    }

    fn h_of(beakers: &[Beaker]) -> u32 {
        beakers.iter().map(|beaker| beaker.h).sum()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.sorted_cord == other.sorted_cord
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.sorted_cord.hash(state);
    }
}
