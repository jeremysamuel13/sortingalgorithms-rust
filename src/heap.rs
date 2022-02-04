use crate::*;

pub struct HeapSort;

impl Sorter for HeapSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        let len = slice.len();
        for i in (0..len / 2).rev() {
            Self::heapify(slice, len, i)
        }

        for i in (0..len).rev() {
            slice.swap(0, i);
            Self::heapify(slice, i, 0)
        }
    }
}

impl HeapSort {
    fn heapify<T>(slice: &mut [T], size: usize, root: usize)
    where
        T: PartialEq + Ord,
    {
        //largest of left, right and root
        let (largest, _) = [2 * root + 1, 2 * root + 2, root]
            .into_iter()
            .flat_map(|x| (x < size).then(|| slice.get(x).map(|y| (x, y)))?)
            .max_by(|(_, x), (_, y)| x.cmp(y))
            .unwrap_or((root, &slice[root]));

        if largest != root {
            slice.swap(root, largest);
            Self::heapify(slice, size, largest);
        }
    }
}
