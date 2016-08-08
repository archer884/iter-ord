#[macro_use] mod try_opt;

mod intersection;
mod difference;
mod distinct;

pub use intersection::{
    AscendingIntersection,
    DescendingIntersection,
};

pub use difference::{
    AscendingDifference,
    DescendingDifference,
};

pub use distinct::{
    AscendingDistinct,
    DescendingDistinct,
};
