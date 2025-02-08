pub const LANE_COUNT: usize = 32;
pub const UNROLL_FACTOR: usize = 4;

mod all;
mod any;
mod argmax;
mod argmin;
mod contains;
mod filter;
mod find;
mod position;

pub use all::AllSimd;
pub use any::AnySimd;
pub use argmax::ArgmaxSimd;
pub use argmax::ArgmaxSimdFast;
pub use argmin::ArgminSimd;
pub use argmin::ArgminSimdFast;
pub use contains::ContainsSimd;
pub use filter::FilterSimd;
pub use find::FindSimd;
pub use position::PositionSimd;
