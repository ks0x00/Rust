#[allow(unused_imports)]
use crate::fast_astar::FastAStar;
#[allow(unused_imports)]
use crate::exaustive_astar::ExaustiveAStar;

// pub type Astar = FastAStar;
pub type Astar = ExaustiveAStar;

pub mod node;
pub mod fast_astar;
pub mod exaustive_astar;