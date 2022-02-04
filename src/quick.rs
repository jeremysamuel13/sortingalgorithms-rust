use crate::*;

pub struct RandQuickSort;

impl Sorter for RandQuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord + Clone,
    {
        quicksort(slice, true)
    }
}

pub struct QuickSort;

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where
        T: PartialEq + Ord + Clone,
    {
        quicksort(slice, false)
    }
}

fn quicksort<T>(slice: &mut [T], rand: bool)
where
    T: PartialEq + Ord + Clone,
{
    match slice.len() {
        2 => (slice[0] > slice[1])
            .then(|| slice.swap(0, 1))
            .unwrap_or(()),
        3.. => {
            if rand {
                //RANDOMIZING PIVOT USING MEDIAN OF 3 RANDOM VALUES
                use rand::Rng;

                let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
                let (a, b, c) = (
                    rng.gen_range(0..slice.len()),
                    rng.gen_range(0..slice.len()),
                    rng.gen_range(0..slice.len()),
                );

                //median
                let pivot = if (slice[a] > slice[b] && slice[a] < slice[c])
                    || (slice[a] >= slice[b] && slice[a] > slice[c])
                {
                    a
                } else if (slice[a] > slice[b] && slice[b] > slice[c])
                    || (slice[a] <= slice[b] && slice[b] < slice[c])
                {
                    b
                } else {
                    c
                };

                //swaps so algorithm can work independent of whether the pivot is chosen randomly or not
                slice.swap(0, pivot);
            }

            //Actual aglorithm
            let (pivot, rest) = slice.split_first_mut().unwrap();
            let mut left = 0;
            let mut right = rest.len() - 1;
            while left <= right {
                if &rest[left] <= pivot {
                    left += 1;
                } else if &rest[right] > pivot {
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                } else {
                    rest.swap(left, right);
                    left += 1;
                    if right == 0 {
                        break;
                    }
                    right -= 1;
                }
            }
            slice.swap(0, left);
            let (left, right) = slice.split_at_mut(left);
            quicksort(left, rand);
            quicksort(&mut right[1..], rand);
        }
        _ => (),
    }
}
