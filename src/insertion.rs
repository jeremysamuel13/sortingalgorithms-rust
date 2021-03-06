use crate::*;

pub struct InsertionSort;

impl<T> Sorter<T> for InsertionSort {
    fn sort(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        for i in 1..slice.len() {
            let mut j = i;
            while j > 0 && slice[j] < slice[j - 1] {
                slice.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}
