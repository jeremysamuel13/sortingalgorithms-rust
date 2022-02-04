use crate::*;

pub struct MergeSort;

impl Sorter for MergeSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord + Clone,
    {
        Self::mergesort(slice, 0, slice.len() - 1)
    }
}

impl MergeSort {
    fn merge<T>(slice: &mut [T], l: usize, m: usize, r: usize)
    where
        T: PartialEq + Ord + Clone,
    {
        let left = &slice[l..=m];
        let right = &slice[m + 1..=r];

        let (mut li, mut ri) = (0, 0);

        let mut sorted: Vec<T> = vec![];

        loop {
            match (li < left.len(), ri < right.len()) {
                (true, true) => {
                    if left[li] <= right[ri] {
                        sorted.push(left[li].clone());
                        li += 1;
                    } else {
                        sorted.push(right[ri].clone());
                        ri += 1;
                    }
                }
                (true, false) => {
                    sorted.extend_from_slice(&left[li..]);
                    li = left.len();
                }
                (false, true) => {
                    sorted.extend_from_slice(&right[ri..]);
                    ri = right.len();
                }
                _ => break,
            }
        }

        for (idx, val) in (l..=r).zip(sorted.into_iter()) {
            slice[idx] = val;
        }
    }

    fn mergesort<T>(slice: &mut [T], l: usize, r: usize)
    where
        T: PartialEq + Ord + Clone,
    {
        if l < r {
            let m = (l + r) / 2;
            Self::mergesort(slice, l, m);
            Self::mergesort(slice, m + 1, r);

            Self::merge(slice, l, m, r);
        }
    }
}
