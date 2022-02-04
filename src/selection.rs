use crate::*;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        for i in 0..slice.len() {
            let (swidx, _) = slice[i..]
                .iter()
                .enumerate()
                .min_by(|(_, x), (_, y)| x.cmp(y))
                .unwrap();
            slice.swap(i, swidx + i);

            //let mut swidx = i;
            // for j in i..slice.len() {
            //     swidx = std::cmp::min_by(j, swidx, |x, y| slice[*x].cmp(&slice[*y]));
            // }
            // slice.swap(i, swidx);
        }
    }
}
