pub struct AscendingDifference<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    left: T1,
    right: T2,
}

impl<T, T1, T2> AscendingDifference<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    pub fn new<I1, I2>(left: I1, right: I2) -> AscendingDifference<T, T1, T2>
        where I1: IntoIterator<IntoIter = T1, Item = T>,
              I2: IntoIterator<IntoIter = T2, Item = T>,
    {
        AscendingDifference {
            left: left.into_iter(),
            right: right.into_iter(),
        }
    }
}

impl<T, T1, T2> Iterator for AscendingDifference<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>
          T2: Iterator<Item = T>
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct DescendingDifference<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    left: T1,
    right: T2,
}
