use crate::basic_range::*;

//use std::ops::{Add, Sub, AddAssign};

// Struct to represent the indexed range
pub struct IndexedRange<T>
where
    T: IteratorOps,
{
    basic_range: BasicRange<T>,
}

impl<T> IndexedRange<T>
where
    T: IteratorOps,
{
    pub fn new(start: T, end: T, step: T::Step, inclusive: bool) -> Self {
        // if step == T::zero() {
        //     panic!("Step cannot be zero");
        // }
        IndexedRange {
            basic_range: BasicRange::<T>::new(start, end, step, inclusive),
        }
    }
}

pub struct IndexedRangeIter<T>
where
    T: IteratorOps,
{
    basic_range_iter: BasicRangeIter<T>,
    index: usize,
}

// Implement the Iterator trait for IndexedRangeIter
impl<T> Iterator for IndexedRangeIter<T>
where
    T: IteratorOps,
{
    type Item = (usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        match self.basic_range_iter.next() {
            Some(t) => {
                let result = (self.index, t);
                self.index += 1;
                Some(result)
            }
            None => None,
        }
    }
}

// Implement IntoIterator for IndexedRange
impl<T> IntoIterator for IndexedRange<T>
where
    T: IteratorOps,
{
    type Item = (usize, T);
    type IntoIter = IndexedRangeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IndexedRangeIter {
            basic_range_iter: BasicRangeIter::<T> {
                current: self.basic_range.start,
                end: self.basic_range.end,
                step: self.basic_range.step,

                inclusive_or_not_on_step: self.basic_range.inclusive_or_not_on_step,
                invalid_range: self.basic_range.invalid_range,
            },
            index: 0,
        }
    }
}

#[macro_export]
macro_rules! indexed_range_exclusive {
    ($typename:ty, $start:expr, $end:expr) => {
        indexed_range_exclusive!($typename, $start, $end, 1)
    };
    ($typename:ty, $start:expr, $end:expr, $step:expr) => {
        IndexedRange::<$typename>::new($start, $end, $step, false)
    };
}

#[macro_export]
macro_rules! indexed_range_inclusive {
    ($typename:ty, $start:expr, $end:expr) => {
        indexed_range_inclusive!($typename, $start, $end, 1)
    };
    ($typename:ty, $start:expr, $end:expr, $step:expr) => {
        IndexedRange::<$typename>::new($start, $end, $step, true)
    };
}

#[cfg(test)]
fn verify_indexed_range<T>(expect: &Vec<T>, r: IndexedRange<T>)
where
    T: IteratorOps + PartialEq + std::fmt::Debug,
    IndexedRange<T>: IntoIterator<Item = (usize, T)>, // Ensures IndexedRange<T> can be converted into an iterator
{
    let mut index = 0;
    for (i, v) in r {
        assert_eq!(i, index, "index not correct");
        assert!(
            i < expect.len(),
            "index out of bounds: len is {} but index is {}",
            expect.len(),
            i
        );
        assert_eq!(v, expect[index], "Value mismatch at index {}", index);
        index += 1;
    }
    assert_eq!(index, expect.len(), "Final length mismatch");
}

#[cfg(test)]
mod main_test {
    use super::*;

    #[test]
    fn basic_1() {
        let expect = vec![0, 1, 2];
        verify_indexed_range(&expect, IndexedRange::new(0, 3, 1, false));
        verify_indexed_range::<i32>(&expect, IndexedRange::<i32>::new(0, 3, 1, false));

        let expect = vec![0, 1, 2];
        verify_indexed_range(&expect, IndexedRange::new(0, 2, 1, true));
    }

    #[test]
    fn basic_2() {
        let expect = vec![0, 1, 2];
        verify_indexed_range(&expect, indexed_range_exclusive!(i32, 0, 3));

        let expect = vec![0, 1, 2];
        verify_indexed_range(&expect, indexed_range_inclusive!(i32, 0, 2));

        let expect = vec![3, 2, 1];
        verify_indexed_range(&expect, IndexedRange::new(3, 0, -1, false));

        let expect = vec![3, 2, 1];
        verify_indexed_range(&expect, indexed_range_exclusive!(i32, 3, 0, -1));

        let expect = vec![3, 2, 1];
        verify_indexed_range(&expect, indexed_range_inclusive!(i32, 3, 1, -1));
    }

    #[test]
    fn basic() {
        let expect = vec![0, 2];
        verify_indexed_range(&expect, IndexedRange::new(0, 4, 2, false));

        let expect = vec![0, 2];
        verify_indexed_range(&expect, indexed_range_exclusive!(i32, 0, 4, 2));

        let expect = vec![0, 2];
        verify_indexed_range(&expect, IndexedRange::new(0, 3, 2, false));

        let expect = vec![0, 2];
        verify_indexed_range(&expect, indexed_range_inclusive!(i32, 0, 2, 2));

        let expect = vec![0, 2];
        verify_indexed_range(&expect, indexed_range_inclusive!(i32, 0, 3, 2));
    }

    #[test]
    //fn void_range_prop(inclusive in 0..=1) {
    fn void_range_prop() {
        for inclusive in 0..=1 {
            let expect_none = vec![];
            let expect_once = vec![0];
            verify_indexed_range(
                if inclusive > 0 {
                    &expect_once
                } else {
                    &expect_none
                },
                IndexedRange::new(0, 0, -2, inclusive > 0),
            );

            let expect_none = vec![];
            let expect_once = vec![0];
            verify_indexed_range(
                if inclusive > 0 {
                    &expect_once
                } else {
                    &expect_none
                },
                IndexedRange::new(0, 0, 2, inclusive > 0),
            );

            let expect = vec![];
            verify_indexed_range(&expect, IndexedRange::new(3, 0, 1, inclusive > 0));

            let expect = vec![];
            verify_indexed_range(&expect, IndexedRange::new(0, 1, -1, inclusive > 0));
        }
    }

    #[test]
    //fn not_on_step_1_prop(inclusive in 0..=1) {
    fn not_on_step_1() {
        for inclusive in 0..=1 {
            let expect = vec![0, 2, 4];
            verify_indexed_range(&expect, IndexedRange::new(0, 5, 2, inclusive > 0));
        }
    }

    #[test]
    fn not_on_step_2() {
        for inclusive in 0..=1 {
            let expect = vec![0, 2, 4];
            verify_indexed_range(
                &expect,
                if 0 == inclusive {
                    indexed_range_exclusive!(i32, 0, 5, 2)
                } else {
                    indexed_range_inclusive!(i32, 0, 5, 2)
                },
            );
        }
    }

    #[test]
    fn not_on_step_3() {
        for inclusive in 0..=1 {
            let expect = vec![5, 3];
            verify_indexed_range(&expect, IndexedRange::new(5, 2, -2, inclusive > 0));
        }
    }

    #[test]
    fn not_on_step_4() {
        for inclusive in 0..=1 {
            let expect = vec![5, 3];
            verify_indexed_range(
                &expect,
                if inclusive > 0 {
                    indexed_range_inclusive!(i32, 5, 2, -2)
                } else {
                    indexed_range_exclusive!(i32, 5, 2, -2)
                },
            );
        }
    }

    #[test]
    #[should_panic]
    fn zero_step() {
        // Infinity or panic
        for (i, value) in IndexedRange::new(0, 1, 0, false) {
            println!("index:{} value:{}", i, value);
        }

        // Infinity or panic
        for (i, value) in IndexedRange::new(0, 1, 0, true) {
            println!("index:{} value:{}", i, value);
        }
    }
}
