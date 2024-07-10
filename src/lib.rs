#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

mod all_equal;
mod contains;
mod eq;
mod filter;
mod find;
mod is_sorted;
mod max;
mod min;
mod position;
pub const SIMD_LEN: usize = 16;

pub use all_equal::AllEqualSimd;
pub use contains::ContainsSimd;
pub use eq::SimdEq;
pub use find::FindSimd;
pub use is_sorted::IsSortedSimd;
pub use max::MaxSimd;
pub use min::MinSimd;
pub use position::PositionSimd;
