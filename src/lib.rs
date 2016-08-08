#![cfg_attr(test, feature(test))]
#[cfg(test)] extern crate test;

#[macro_use] mod try_opt;

mod intersection;

pub use intersection::{
    AscendingIntersection,
    DescendingIntersection,
};
