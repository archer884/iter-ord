use std::cmp::{Eq, Ord};

trait AscendingIterator: Iterator
    where <Self as Iterator>::Item: Eq + Ord
{
}

trait DescendingIterator: Iterator
    where <Self as Iterator>::Item: Eq + Ord
{
}
