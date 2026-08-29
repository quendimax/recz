#![cfg_attr(not(test), no_std)]

mod label;
pub use label::Label;

pub mod str;

pub use recz_macro::__re as re;
