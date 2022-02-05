use std::collections::{BinaryHeap, VecDeque};

use num::{CheckedAdd, CheckedDiv, Integer, NumCast};
use std::cmp::Reverse;

use crate::*;

//With binary heap
//Binary heap allows for each bucket to remain sorted
pub struct BHBucketSort;

impl<T> Sorter<T> for BHBucketSort
where
    T: Integer + CheckedDiv + CheckedAdd + NumCast + Clone + Copy,
{
    fn sort(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        match slice.len() {
            2 => (slice[0] > slice[1])
                .then(|| slice.swap(0, 1))
                .unwrap_or(()),
            3.. => {
                let num_of_buckets = (slice.len() as f64).sqrt() as usize + 1;
                let max = slice.iter().max().unwrap();
                let min = slice.iter().min().unwrap();

                let mut buckets = BHBucketedVec::new(*min, *max, num_of_buckets);

                for val in slice.iter() {
                    buckets.insert(*val);
                }

                let mut i = 0;

                while let Some(v) = buckets.pop_front() {
                    slice[i] = v;
                    i += 1;
                }
            }
            _ => (),
        }
    }
}

struct BHBucketedVec<T>
where
    T: Integer + CheckedDiv + CheckedAdd + NumCast + Clone + Copy,
{
    buckets: VecDeque<BinaryHeap<Reverse<T>>>,
    range: T,
    min: T,
}

impl<T> BHBucketedVec<T>
where
    T: Integer + CheckedDiv + CheckedAdd + NumCast + Clone + Copy,
{
    fn new(min: T, max: T, num_of_buckets: usize) -> Self {
        let range = (max - min) / T::from(num_of_buckets).unwrap();

        Self {
            buckets: num::range_step(min, max + num::cast(1).unwrap(), range)
                .map(|_| BinaryHeap::<Reverse<T>>::new())
                .collect(),
            range,
            min,
        }
    }

    fn insert(&mut self, val: T) {
        let norm = val - self.min;
        let idx: usize = num::cast(norm / self.range).unwrap();
        self.buckets[idx].push(Reverse(val));
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.buckets.is_empty() {
            None
        } else if self.buckets[0].is_empty() {
            self.buckets.pop_front();
            self.pop_front()
        } else {
            self.buckets[0].pop().map(|x| x.0)
        }
    }
}

//With vec and generic across sorting algorithms that implement the "Sorter" trait for the sorting of each bucket
//Some form of radix sort seems to be the best idea since these "buckets" are designed to be smaller vecs of data
pub struct BucketSort<S> {
    _sorter: S,
}

impl<T, S> Sorter<T> for BucketSort<S>
where
    S: Sorter<T>,
    T: Integer + CheckedDiv + CheckedAdd + NumCast + Clone + Copy,
{
    fn sort(slice: &mut [T])
    where
        T: PartialEq + Ord,
    {
        match slice.len() {
            2 => (slice[0] > slice[1])
                .then(|| slice.swap(0, 1))
                .unwrap_or(()),
            3.. => {
                let num_of_buckets = (slice.len() as f64).sqrt() as usize + 1;
                let max = slice.iter().max().unwrap();
                let min = slice.iter().min().unwrap();

                let mut buckets = BucketedVec::new(*min, *max, num_of_buckets);

                for val in slice.iter() {
                    buckets.insert(*val);
                }

                buckets.sort_buckets::<S>();

                let mut i = 0;

                while let Some(vec) = buckets.pop_front_vec() {
                    for val in vec {
                        slice[i] = val;
                        i += 1;
                    }
                }
            }
            _ => (),
        }
    }
}

struct BucketedVec<T>
where
    T: Integer + CheckedDiv + CheckedAdd + NumCast + Clone + Copy,
{
    buckets: VecDeque<Vec<T>>,
    range: T,
    min: T,
}

impl<T> BucketedVec<T>
where
    T: Integer + CheckedDiv + CheckedAdd + NumCast + Clone + Copy,
{
    fn new(min: T, max: T, num_of_buckets: usize) -> Self {
        let range = (max - min) / T::from(num_of_buckets).unwrap();

        Self {
            buckets: num::range_step(min, max + num::cast(1).unwrap(), range)
                .map(|_| Vec::new())
                .collect(),
            range,
            min,
        }
    }

    fn insert(&mut self, val: T) {
        let norm = val - self.min;
        let idx: usize = num::cast(norm / self.range).unwrap();
        self.buckets[idx].push(val);
    }

    fn sort_buckets<S: Sorter<T>>(&mut self) {
        for i in 0..self.buckets.len() {
            S::sort(&mut self.buckets[i]);
        }
    }

    fn pop_front_vec(&mut self) -> Option<Vec<T>> {
        if self.buckets.is_empty() {
            None
        } else {
            self.buckets.pop_front()
        }
    }
}
