use crate::*;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        for i in 1..slice.len() {
            let mut j = i;
            while slice[j] < slice[j - 1] && j > 0 {
                slice.swap(j, j - 1);
                j -= 1;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut v = vec![0, 3, 2, 12, 5, 3, 9];
    InsertionSort::sort(&mut v);
    assert_eq!(v, vec![0, 2, 3, 3, 5, 9, 12]);
}
