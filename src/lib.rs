/*
    This structure was inspired from John Gjengset's Crust of Rust livestream on sorting algorithms, check it out:
    https://www.youtube.com/watch?v=h4RkCyJyXmM
*/

use std::fmt::Debug;

pub mod bubble;
pub mod insertion;
pub mod min;

pub mod merge;

pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord + Clone;
}

pub trait SorterBenchmark: Sorter {
    fn sort_bench<T>(slice: &mut [T])
    where
        T: PartialEq + Ord;
}

pub struct StdSorter;

impl Sorter for StdSorter {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        slice.sort();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let mut v = vec![0, 3, 2, 12, 5, 3, 9];
        StdSorter::sort(&mut v);
        assert_eq!(v, vec![0, 2, 3, 3, 5, 9, 12]);
    }
}