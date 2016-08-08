use std::cmp::{Ord, Ordering};

pub struct AscendingIntersection<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    left: T1,
    right: T2,
}

impl<T, T1, T2> AscendingIntersection<T, T1, T2> 
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    pub fn new<I1, I2>(left: I1, right: I2) -> AscendingIntersection<T, T1, T2> 
        where I1: IntoIterator<IntoIter = T1, Item = T>,
              I2: IntoIterator<IntoIter = T2, Item = T>,
    {
        AscendingIntersection {
            left: left.into_iter(),
            right: right.into_iter(),
        }
    }
}

impl<T, T1, T2> Iterator for AscendingIntersection<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut left = try_opt!(self.left.next());
        let mut right = try_opt!(self.right.next());

        loop {
            match left.cmp(&right) {
                Ordering::Equal => return Some((left, right)),
                Ordering::Less => left = try_opt!(self.left.next()),
                Ordering::Greater => right = try_opt!(self.right.next()),
            }
        }
    }
}

pub struct DescendingIntersection<T, T1, T2>
    where T: Eq + Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    left: T1,
    right: T2,
}

impl<T, T1, T2> DescendingIntersection<T, T1, T2>
    where T: Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
     pub fn new<I1, I2>(left: I1, right: I2) -> DescendingIntersection<T, T1, T2> 
        where I1: IntoIterator<IntoIter = T1, Item = T>,
              I2: IntoIterator<IntoIter = T2, Item = T>,
    {
        DescendingIntersection {
            left: left.into_iter(),
            right: right.into_iter(),
        }
    }
}

impl<T, T1, T2> Iterator for DescendingIntersection<T, T1, T2>
    where T: Eq + Ord,
          T1: Iterator<Item = T>,
          T2: Iterator<Item = T>,
{
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut left = try_opt!(self.left.next());
        let mut right = try_opt!(self.right.next());

        loop {
            match left.cmp(&right) {
                Ordering::Equal => return Some((left, right)),
                Ordering::Less => right = try_opt!(self.right.next()),
                Ordering::Greater => left = try_opt!(self.left.next()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use test::{self, Bencher};
    use intersection::*;

    #[test]
    fn ascending_intersection_works() {
        let a = &[1, 2, 3, 4, 5];
        let b = &[2, 4, 6];
        let c: Vec<_> = AscendingIntersection::new(a, b)
            .map(|(&a, _)| a)
            .collect();

        assert_eq!([2, 4], c.as_ref());
    }

    #[test]
    fn descending_intersection_works() {
        let a = &[5, 4, 3, 2, 1];
        let b = &[6, 4, 2];
        let c: Vec<_> = DescendingIntersection::new(a, b)
            .map(|(&a, _)| a)
            .collect();

        assert_eq!([4, 2], c.as_ref());
    }

    #[bench]
    fn intersect_by_hash_set(bencher: &mut Bencher) {
        use std::collections::HashSet;

        let a: Vec<_> = (1..1000).collect();
        let b: Vec<_> = (1..1000).filter(|n| n & 1 == 0).collect();

        bencher.iter(|| {
            let a: HashSet<_> = a.iter().cloned().collect();
            let b: HashSet<_> = b.iter().cloned().collect();
            let c: Vec<_> = a.intersection(&b).cloned().collect();

            test::black_box(c)
        });
    }

    #[bench]
    fn intersect_by_ascending(bencher: &mut Bencher) {
        let a: Vec<_> = (1..1000).collect();
        let b: Vec<_> = (1..1000).filter(|n| n & 1 == 0).collect();
        
        bencher.iter(|| {
            let result: Vec<_> = AscendingIntersection::new(a.iter(), b.iter()).map(|(&a, _)| a).collect();

            test::black_box(result);
        })
    }
}
