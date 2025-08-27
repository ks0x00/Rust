// pub const NONE: i32 = -1;
pub const UNDEFINED_U32: u32 = u32::MAX;
pub const INFINITY_USIZE: usize = usize::MAX;
pub const MAX_WATERS: usize = 4;

pub type Water = u32;
pub type Cord = Vec<u32>;

pub mod beaker;
pub mod state;
