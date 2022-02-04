/*
    This structure was inspired from John Gjengset's Crust of Rust livestream on sorting algorithms, check it out:
    https://www.youtube.com/watch?v=h4RkCyJyXmM
*/

pub mod bubble;
pub mod heap;
pub mod insertion;
pub mod merge;
pub mod quick;
pub mod selection;

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
    use crate::{
        bubble::BubbleSort,
        heap::HeapSort,
        insertion::InsertionSort,
        merge::MergeSort,
        quick::{QuickSort, RandQuickSort},
        selection::SelectionSort,
    };

    macro_rules! sorting_tests {
        ($($name:ident: $type:ty,)*) => {
        $(
            mod $name {
                use super::*;
                use crate::Sorter;

                #[test]
                fn test() {
                    let mut rv = vec![0, 3, 2, 12, 5, 3, 9];
                    <$type>::sort(&mut rv);
                    assert_eq!(rv, vec![0, 2, 3, 3, 5, 9, 12]);
                }

                #[test]
                fn random() {
                    use rand::Rng;
                    let mut rv: Vec<u64> = vec![];
                    let mut rng = rand::thread_rng();

                    for _ in 0..100 {
                        rv.push(rng.gen());
                    }

                    <$type>::sort(&mut rv);
                    for i in 1..rv.len() {
                        assert!(rv[i] > rv[i-1]);
                    }
                }

                #[test]
                fn empty() {
                    let mut empty: Vec<u32> = vec![];
                    <$type>::sort(&mut empty);
                    assert_eq!(empty, &[]);
                }

                #[test]
                fn already_sorted() {
                    let mut sorted: Vec<i8> = (-50..50).collect();
                    <$type>::sort(&mut sorted);
                    assert_eq!(sorted, (-50..50).collect::<Vec<i8>>());
                }
            }
        )*
        }
    }
    sorting_tests! {
        merge: MergeSort,
        bubble: BubbleSort,
        heap: HeapSort,
        insertion: InsertionSort,
        selection: SelectionSort,
        quick: QuickSort,
        randquick: RandQuickSort,
    }
}
