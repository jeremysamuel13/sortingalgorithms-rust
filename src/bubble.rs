use crate::*;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(slice: &mut [T])
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
