use std::cmp::{Eq, Ord};

pub trait AscendingIterator: Iterator
    where <Self as Iterator>::Item: Eq + Ord
{
}

pub trait DescendingIterator: Iterator
    where <Self as Iterator>::Item: Eq + Ord
{
}
