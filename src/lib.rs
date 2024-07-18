#![feature(portable_simd)]
#![feature(is_sorted)]
#![feature(sort_floats)]

pub const SIMD_LEN: usize = 32;

mod all_equal;
mod contains;
mod eq;
mod filter;
mod find;
mod is_sorted;
mod max;
mod min;
mod minmax;
mod position;

pub use all_equal::AllEqualSimd;
pub use contains::ContainsSimd;
pub use eq::EqSimd;
pub use filter::FilterSimd;
pub use find::FindSimd;
pub use is_sorted::IsSortedSimd;
pub use max::MaxSimd;
pub use min::MinSimd;
pub use minmax::MinMaxSimd;
pub use position::PositionSimd;
