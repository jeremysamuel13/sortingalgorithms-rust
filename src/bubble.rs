use crate::*;

pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..(slice.len()) {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}
