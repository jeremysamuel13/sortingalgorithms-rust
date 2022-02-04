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
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut v = vec![0, 3, 2, 12, 5, 3, 9];
    BubbleSort::sort(&mut v);
    assert_eq!(v, vec![0, 2, 3, 3, 5, 9, 12]);
}
