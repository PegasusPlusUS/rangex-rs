use std::mem;
use std::ops::*;

pub trait StepOps:
    Add<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Div<Output = Self>
    + Sub<Output = Self>
    + Copy
    + PartialOrd
    + PartialEq
    + std::fmt::Debug
    + std::fmt::Display
{
    fn zero() -> Self; // { 0 as Self }
    fn one() -> Self; // { 1 }
    fn negative_one() -> Self; // { -1 }
    fn rem_euclid(self, rhs: Self) -> Self { self % rhs }
    fn floor(self) -> Self { self }
    fn abs(self) -> Self { if self < Self::zero() { Self::zero() - self } else { self } }
    //fn from_const(v: f64) -> Self { v as Self }
}

impl StepOps for i128 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn negative_one() -> Self {
        -1
    }
}
impl StepOps for i64 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn negative_one() -> Self {
        -1
    }
}
impl StepOps for i32 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn negative_one() -> Self {
        -1
    }
}
impl StepOps for i16 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn negative_one() -> Self {
        -1
    }
}
impl StepOps for i8 {
    fn zero() -> Self {
        0
    }
    fn one() -> Self {
        1
    }
    fn negative_one() -> Self {
        -1
    }
}

impl StepOps for f32 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn negative_one() -> Self { -1.0 }
    fn floor(self) -> Self { self.floor() }
}

impl StepOps for f64 {
    fn zero() -> Self { 0.0 }
    fn one() -> Self { 1.0 }
    fn negative_one() -> Self { -1.0 }
    fn floor(self) -> Self { self.floor() }
}

pub trait IteratorOps:
    Add<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    //+ Neg<Output = Self>
    + AddAssign
    + Copy
    + PartialOrd
    + PartialEq
    + std::fmt::Debug
    + std::fmt::Display
{
    type Step: StepOps;
    fn to_step(self) -> Self::Step; // { self }
    fn from_step(step: Self::Step) -> Self; // { step } // Conversion back to original type
}

pub trait SizeCompatible<T> {}

// Implement `SizeCompatible` only for types with the same size.
impl SizeCompatible<u8> for i8 {}
impl SizeCompatible<i8> for i8 {}
impl SizeCompatible<u16> for i16 {}
impl SizeCompatible<i16> for i16 {}
impl SizeCompatible<u32> for i32 {}
impl SizeCompatible<i32> for i32 {}
impl SizeCompatible<u64> for i64 {}
impl SizeCompatible<i64> for i64 {}
impl SizeCompatible<u128> for i128 {}
impl SizeCompatible<i128> for i128 {}

// Implement the trait for specific types
impl IteratorOps for u8 {
    type Step = i8;

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }

    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }
}
impl IteratorOps for i8 {
    type Step = i8;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}
impl IteratorOps for u16 {
    type Step = i16;

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }

    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }
}
impl IteratorOps for i16 {
    type Step = i16;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}
impl IteratorOps for u32 {
    type Step = i32;

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }

    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }
}
impl IteratorOps for i32 {
    type Step = i32;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}
impl IteratorOps for u64 {
    type Step = i64;

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }

    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }
}
impl IteratorOps for i64 {
    type Step = i64;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}
impl IteratorOps for u128 {
    type Step = i128;

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }

    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }
}
impl IteratorOps for i128 {
    type Step = i128;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}

impl IteratorOps for f32 {
    type Step = f32;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}

impl IteratorOps for f64 {
    type Step = f64;

    fn to_step(self) -> Self::Step {
        self
    }

    fn from_step(step: Self::Step) -> Self {
        step
    }
}

pub struct BasicRange<T>
where
    T: IteratorOps,
{
    pub start: T,
    pub end: T,
    pub step: T::Step,
}

impl<T> BasicRange<T>
where
    T: IteratorOps + std::fmt::Display,
    T::Step: std::fmt::Display,
{
    pub fn new(start: T, mut end: T, step: T::Step, mut inclusive: bool) -> Self {
        if step == T::Step::zero() {
            panic!("Step can't be 0");
        }

        let range_size = if step > T::Step::zero() {
            end - start
        } else {
            start - end
        };

        let on_step;
        if start != end {
            if range_size > T::from_step(T::Step::zero()) {
                if step < T::Step::negative_one() || T::Step::one() < step {
                    // Use specialized functions based on type
                    (end, on_step) = Self::calculate_stop_and_steps(start, end, range_size, step)
                    //} else {
                    //    Self::calculate_integer_steps(start, end, range_size, step)
                    //}
                    ;
                    println!("end is {}, zero is {}, step is {}, on_step is {}", end, T::Step::zero(), step, on_step);
                } else {
                    on_step = true;
                }
            } else {
                on_step = true;
                inclusive = false;
            }

            if inclusive || !on_step {
                println!("end is {}, zero is {}, step is {}, on_step is {}", end, T::Step::zero(), step, on_step);
                end = T::from_step(end.to_step() + step);
                println!("end is {}, zero is {}, step is {}, on_step is {}", end, T::Step::zero(), step, on_step);
            }
        }
        
        BasicRange { start, end, step }
    }

    fn calculate_stop_and_steps(start: T, end: T, range_size: T, step: T::Step) -> (T, bool)
    where
        T::Step: StepOps
    {
        //println!("start {} end {} range_size {} step {}", start, end, range_size, step);
        let range_size_as_step = T::to_step(range_size);
        let positive_step :T::Step = if step < T::Step::zero() { T::Step::zero() - step } else { step};
        let steps = (range_size_as_step / positive_step).floor();
        //println!("range_size_as_step {} steps {}", range_size_as_step, steps);
        let on_step = T::Step::zero() == range_size_as_step.rem_euclid(step);
        let new_range_size = T::from_step(steps * positive_step);
        let new_end : T::Step = start.to_step() + if start < end {
            new_range_size.to_step()
        } else {
            T::Step::zero() - new_range_size.to_step()
        };
        //println!("new end {}", new_end);
        (T::from_step(new_end), on_step)
    }
}

pub struct BasicRangeIter<T>
where
    T: IteratorOps,
{
    pub current: T,
    pub end: T,
    pub step: T::Step,
}

impl<T> Iterator for BasicRangeIter<T>
where
    T: IteratorOps,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        //println!("Current {}, step {}, end {}", self.current, self.step, self.end);
        let stop = if self.step > T::Step::zero() {
            self.current >= self.end
        } else {
            self.current <= self.end
        };

        if stop {
            None
        } else {
            let result = self.current;
            self.current = T::from_step(self.current.to_step() + self.step);
            Some(result)
        }
    }
}

impl<T> IntoIterator for BasicRange<T>
where
    T: IteratorOps,
{
    type Item = T;
    type IntoIter = BasicRangeIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        BasicRangeIter {
            current: self.start,
            end: self.end,
            step: self.step,
        }
    }
}

#[macro_export]
macro_rules! range_exclusive {
    ($start:expr, $end:expr) => {
        $start..$end
    };
    ($typename:ty, $start:expr, $end:expr) => {
        range_exclusive!($typename, $start, $end, 1)
    };
    ($typename:ty, $start:expr, $end:expr, $step:expr) => {
        BasicRange::<$typename>::new($start, $end, $step, false)
    };
}

#[macro_export]
macro_rules! range_inclusive {
    ($start:expr, $end:expr) => {
        $start..=$end
    };
    ($typename:ty, $start:expr, $end:expr) => {
        range_inclusive!($typename, $start, $end, 1)
    };
    ($typename:ty, $start:expr, $end:expr, $step:expr) => {
        BasicRange::<$typename>::new($start, $end, $step, true)
    };
}

#[cfg(test)]
mod main_test {
    use super::*;

    fn verify_range<T>(expect: Vec<T>, r: BasicRange<T>)
    where
        T: IteratorOps + std::fmt::Display,
        <BasicRange<T> as IntoIterator>::Item: std::fmt::Debug + PartialEq<T>,
    {
        let mut index = 0;
        for value in r {
            assert_eq!(value, expect[index]);
            //println!("index:{} value:{}", index, value);
            index += 1;
        }
        assert_eq!(index, expect.len());
    }

    fn verify_std_range<T, R>(expect: Vec<T>, r: R)
    where
        T: std::fmt::Debug + std::fmt::Display + PartialEq,
        R: IntoIterator<Item = T>,
    {
        let mut index = 0;
        for value in r {
            assert_eq!(value, expect[index]);
            //println!("index:{} value:{}", index, value);
            index += 1;
        }
        assert_eq!(index, expect.len());
    }

    #[test]
    fn basic_1() {
        let expect = vec![0, 1, 2];
        verify_range(expect, BasicRange::new(0, 3, 1, false));

        let expect = vec![0, 1, 2];
        verify_range(expect, BasicRange::new(0, 2, 1, true));
    }

    #[test]
    fn basic_1_float() {
        let expect = vec![0.0, 1.0, 2.0];
        verify_range(expect, BasicRange::new(0.0, 3.0, 1.0, false));

        let expect = vec![0.0, 1.0, 2.0];
        verify_range(expect, BasicRange::new(0.0, 2.0, 1.0, true));
    }

    #[test]
    fn basic_2() {
        let expect = vec![0, 1, 2];
        verify_std_range(expect, range_exclusive!(0, 3));

        let expect = vec![0, 1, 2];
        verify_std_range(expect, range_inclusive!(0, 2));

        let expect = vec![3, 2, 1];
        verify_range(expect, BasicRange::new(3, 0, -1, false));

        let expect = vec![3, 2, 1];
        verify_range(expect, range_exclusive!(i32, 3, 0, -1));

        let expect = vec![3, 2, 1];
        verify_range(expect, range_inclusive!(i32, 3, 1, -1));
    }
    #[test]
    fn basic_2_float() {
        let expect = vec![0.0, 1.0, 2.0];
        verify_std_range(expect, range_exclusive!(f32, 0.0, 3.0, 1.0));

        let expect = vec![0.0, 1.0, 2.0];
        verify_std_range(expect, range_inclusive!(f32, 0.0, 2.0, 1.0));

        let expect = vec![3.0, 2.0, 1.0];
        verify_range(expect, BasicRange::new(3.0, 0.0, -1.0, false));

        let expect = vec![3.0, 2.0, 1.0];
        verify_range(expect, range_exclusive!(f32, 3.0, 0.0, -1.0));

        let expect = vec![3.0, 2.0, 1.0];
        verify_range(expect, range_inclusive!(f32, 3.0, 1.0, -1.0));
    }
    //}

    #[test]
    fn std_notation() {
        let mut s = 0;
        for v in 3..0 {
            s += v;
        }
        assert_eq!(s, 0);

        let mut s = 0;
        let elements = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        for (index, element) in elements.iter().enumerate() {
            //println!("Index: {}, Element: {}", index, element);
            assert!(index < elements.len());
            assert_eq!(*element, elements[index]);
            s += *element;
        }
        assert_eq!(s, elements.iter().sum());

        let mut s = 0.0;
        let elements = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        for (index, element) in elements.iter().enumerate() {
            //println!("Index: {}, Element: {}", index, element);
            assert!(index < elements.len());
            assert_eq!(*element, elements[index]);
            s += *element;
        }
        assert_eq!(s, elements.iter().sum());
    }

    mod success {
        use super::*;
        //use proptest::prelude::*; // randomly test if used

        //proptest ! {
        #[test]
        //fn void_range_prop(inclusive in 0..=1) {
        fn void_range_prop() {
            for inclusive in 0..=1 {
                let expect = vec![];
                verify_range(expect, BasicRange::new(0, 0, -2, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(0, 0, 2, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(3, 0, 1, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(0, 1, -1, inclusive > 0));
            }
        }
        #[test]
        fn void_range_prop_float() {
            for inclusive in 0..=1 {
                let expect = vec![];
                verify_range(expect, BasicRange::new(0.0, 0.0, -2.0, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(0.0, 0.0, 2.0, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(3.0, 0.0, 1.0, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(0.0, 1.0, -1.0, inclusive > 0));
            }
        }
        #[test]
        //fn not_on_step_1_prop(inclusive in 0..=1) {
        fn not_on_step_1() {
            for inclusive in 0..=1 {
                let expect = vec![0, 2, 4];
                verify_range(expect, BasicRange::new(0, 5, 2, inclusive > 0));
            }
        }
        //}
        #[test]
        //fn not_on_step_1_prop(inclusive in 0..=1) {
        fn not_on_step_1_float() {
            for inclusive in 0..=1 {
                let expect = vec![0.0, 2.0, 4.0];
                verify_range(expect, BasicRange::new(0.0, 5.0, 2.0, inclusive > 0));
            }
        }
        #[test]
        fn not_on_step_2() {
            for inclusive in 0..=1 {
                let expect = vec![0, 2, 4];
                verify_std_range(
                    expect,
                    if 0 == inclusive {
                        range_exclusive!(i32, 0, 5, 2)
                    } else {
                        range_inclusive!(i32, 0, 5, 2)
                    },
                );
            }
        }
        #[test]
        fn not_on_step_2_float() {
            for inclusive in 0..=1 {
                let expect = vec![0.0, 2.0, 4.0];
                verify_std_range(
                    expect,
                    if 0 == inclusive {
                        range_exclusive!(f32, 0.0, 5.0, 2.0)
                    } else {
                        range_inclusive!(f32, 0.0, 5.0, 2.0)
                    },
                );
            }
        }

        #[test]
        fn not_on_step_3() {
            for inclusive in 0..=1 {
                let expect = vec![5, 3];
                verify_range(expect, BasicRange::new(5, 2, -2, inclusive > 0));
            }
            for inclusive in 0..=1 {
                let expect = vec![5, 3];
                verify_range(expect, BasicRange::<u32>::new(5, 2, -2, inclusive > 0));
            }
        }

        #[test]
        fn not_on_step_3_float() {
            for inclusive in 0..=1 {
                let expect = vec![5.0, 3.0];
                verify_range(expect, BasicRange::<f32>::new(5.0, 2.0, -2.0, inclusive > 0));
            }
            for inclusive in 0..=1 {
                let expect = vec![5.0, 3.0];
                verify_range(expect, BasicRange::<f64>::new(5.0, 2.0, -2.0, inclusive > 0));
            }
        }
        #[test]
        fn not_on_step_4() {
            for inclusive in 0..=1 {
                let expect = vec![5, 3];
                verify_range(
                    expect,
                    if inclusive > 0 {
                        range_inclusive!(i32, 5, 2, -2)
                    } else {
                        range_exclusive!(i32, 5, 2, -2)
                    },
                );
            }
        }

        #[test]
        fn not_on_step_4_float() {
            for inclusive in 0..=1 {
                let expect = vec![5.0, 3.0];
                verify_range(
                    expect,
                    if inclusive > 0 {
                        range_inclusive!(f32, 5.0, 2.0, -2.0)
                    } else {
                        range_exclusive!(f32, 5.0, 2.0, -2.0)
                    },
                );
            }
        }
    }

    mod fail {
        use super::*;
        // This causes zero_step() run multiple times #[test]#[test]

        #[test]
        #[ignore]
        fn zero_step() {
            let test_setting = std::env::var("TEST_KEY").unwrap_or_else(|_| "".to_string());
            let mut skip_fail = false;
            let _test_flags: Vec<String> = test_setting
                .split_whitespace()
                .map(|s| {
                    skip_fail = s == "skip_fail";
                    s.to_string()
                })
                .collect();

            if !skip_fail {
                // Infinity or panic
                for _value in BasicRange::new(0, 1, 0, false) {
                    //println!("{}", _value);
                }

                // Infinity or panic
                for _value in BasicRange::new(0, 1, 0, true) {
                    //println!("{}", _value);
                }

                // Infinity or panic
                for _value in BasicRange::new(0.0, 1.0, 0.0, false) {
                    //println!("{}", _value);
                }

                // Infinity or panic
                for _value in BasicRange::new(0.0, 1.0, 0.0, true) {
                    //println!("{}", _value);
                }
            }
        }
    }
}