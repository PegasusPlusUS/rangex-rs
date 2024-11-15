use std::mem;
use std::ops::*;
use num::{Num, Zero, One, FromPrimitive};

pub const DEBUG_PRINT: bool = false;
pub const ERROR_PRINT: bool = true;

pub trait StepOps:
    Num
    //+ num_traits::ops::overflowing::OverflowingAdd
    + PartialOrd
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
{
    fn negative_one() -> Self { Self::zero() - Self::one() }
    fn floor(self) -> Self { self }
    fn abs(self) -> Self { if self < Self::zero() { Self::zero() - self } else { self } }
    fn to_usize(self) -> usize;
}

impl StepOps for isize {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for i128 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for i64 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for i32 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for i16 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for i8 {
    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for f32 {
    fn floor(self) -> Self {
        self.floor()
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

impl StepOps for f64 {
    fn floor(self) -> Self {
        self.floor()
    }

    fn to_usize(self) -> usize {
        self as usize
    }
}

// #![feature(f128_type)]
// impl StepOps for f128 {
//     fn zero() -> Self { 0.0 }
//     fn one() -> Self { 1.0 }
//     fn negative_one() -> Self { -1.0 }
//     fn floor(self) -> Self { self.floor() }
// }

pub trait IteratorOps:
    Num
    + PartialOrd
    + Copy
    + std::fmt::Debug
    + std::fmt::Display
{
    type Step: StepOps;
    type ExtendedStep: StepOps;
    fn min() -> Self;
    fn max() -> Self;
    fn to_step(self) -> Self::Step;
    fn from_step(step: Self::Step) -> Self;
    fn to_extended_step(self) -> Self::ExtendedStep;
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self;
    fn extend_step(step: Self::Step) -> Self::ExtendedStep;
    //fn next(&mut self, step: Self::Step) { *self = *self + Self::from_step(step); }
    fn next(&mut self, step: Self::Step) {
        *self = Self::from_extended_step(self.to_extended_step() + Self::extend_step(step));
    }
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
    type ExtendedStep = i16;

    fn min() -> Self {
        u8::MIN
    }
    fn max() -> Self {
        u8::MAX
    }

    fn to_step(self) -> i8 {
        unsafe { mem::transmute(self) }
    }

    fn from_step(step: i8) -> u8 {
        unsafe { mem::transmute(step) }
    }

    fn to_extended_step(self) -> i16 {
        self as i16
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        if let Some(result) = Self::from_i16(extended_step) {
            if DEBUG_PRINT {
                println!("From extended step {} {}", extended_step, result);
            }
            result
        } else {
            let self_range_inclusive_number = 1 + Self::MAX as Self::ExtendedStep - Self::MIN as Self::ExtendedStep;
            let result =
            if extended_step > Self::MAX as Self::ExtendedStep {
                (extended_step - self_range_inclusive_number) as Self
            } else { 
                (self_range_inclusive_number + extended_step) as Self
            };            
            if DEBUG_PRINT || ERROR_PRINT {
                println!("From extended step overflow {} adjust with {} as {}", extended_step, self_range_inclusive_number, result);
            }
            result
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for i8 {
    type Step = i8;
    type ExtendedStep = i16;

    fn min() -> Self {
        i8::MIN
    }
    fn max() -> Self {
        i8::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        if let Some(result) = Self::from_i16(extended_step) {
            if DEBUG_PRINT {
                println!("From extended step {} {}", extended_step, result);
            }
            result
        } else {
            let self_range_inclusive_number = 1 + Self::MAX as Self::ExtendedStep - Self::MIN as Self::ExtendedStep;
            let result =
            if extended_step > Self::MAX as Self::ExtendedStep {
                (extended_step - self_range_inclusive_number) as Self
            } else { 
                (self_range_inclusive_number + extended_step) as Self
            };            
            if DEBUG_PRINT || ERROR_PRINT {
                println!("From extended step overflow {} adjust with {} as {}", extended_step, self_range_inclusive_number, result);
            }
            result
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for u16 {
    type Step = i16;
    type ExtendedStep = i32;

    fn min() -> Self {
        u16::MIN
    }
    fn max() -> Self {
        u16::MAX
    }

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }
    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        if let Some(result) = Self::from_i32(extended_step) {
            if DEBUG_PRINT {
                println!("From extended step {} {}", extended_step, result);
            }
            result
        } else {
            let self_range_inclusive_number = 1 + Self::MAX as Self::ExtendedStep - Self::MIN as Self::ExtendedStep;
            let result =
            if extended_step > Self::MAX as Self::ExtendedStep {
                (extended_step - self_range_inclusive_number) as Self
            } else { 
                (self_range_inclusive_number + extended_step) as Self
            };            
            if DEBUG_PRINT || ERROR_PRINT {
                println!("From extended step overflow {} adjust with {} as {}", extended_step, self_range_inclusive_number, result);
            }
            result
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for i16 {
    type Step = i16;
    type ExtendedStep = i32;

    fn min() -> Self {
        i16::MIN
    }
    fn max() -> Self {
        i16::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        if let Some(result) = Self::from_i32(extended_step) {
            if DEBUG_PRINT {
                println!("From extended step {} {}", extended_step, result);
            }
            result
        } else {
            let self_range_inclusive_number = 1 + Self::MAX as Self::ExtendedStep - Self::MIN as Self::ExtendedStep;
            let result =
            if extended_step > Self::MAX as Self::ExtendedStep {
                (extended_step - self_range_inclusive_number) as Self
            } else { 
                (self_range_inclusive_number + extended_step) as Self
            };            
            if DEBUG_PRINT || ERROR_PRINT {
                println!("From extended step overflow {} adjust with {} as {}", extended_step, self_range_inclusive_number, result);
            }
            result
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for u32 {
    type Step = i32;
    type ExtendedStep = i64;

    fn min() -> Self {
        u32::MIN
    }
    fn max() -> Self {
        u32::MAX
    }

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }
    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(step: Self::ExtendedStep) -> Self {
        if let Some(result) = u32::from_i64(step) {
            result
        } else {
            0
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for i32 {
    type Step = i32;
    type ExtendedStep = i64;

    fn min() -> Self {
        i32::MIN
    }
    fn max() -> Self {
        i32::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(step: Self::ExtendedStep) -> Self {
        if let Some(result) = i32::from_i64(step) {
            result
        } else {
            0
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for u64 {
    type Step = i64;
    type ExtendedStep = i128;

    fn min() -> Self {
        u64::MIN
    }
    fn max() -> Self {
        u64::MAX
    }

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }
    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(step: Self::ExtendedStep) -> Self {
        if let Some(result) = u64::from_i128(step) {
            result
        } else {
            0
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for i64 {
    type Step = i64;
    type ExtendedStep = i128;

    fn min() -> Self {
        i64::MIN
    }
    fn max() -> Self {
        i64::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as Self::ExtendedStep
    }
    fn from_extended_step(step: Self::ExtendedStep) -> Self {
        if let Some(result) = i64::from_i128(step) {
            result
        } else {
            0
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for u128 {
    type Step = i128;
    type ExtendedStep = i128;

    fn min() -> Self {
        u128::MIN
    }
    fn max() -> Self {
        u128::MAX
    }

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }
    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        unsafe { mem::transmute(self) }
    }
    fn from_extended_step(step: Self::ExtendedStep) -> Self {
        unsafe { mem::transmute(step) }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for i128 {
    type Step = i128;
    type ExtendedStep = i128;

    fn min() -> i128 {
        i128::MIN
    }
    fn max() -> i128 {
        i128::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        extended_step
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for usize {
    type Step = isize;
    type ExtendedStep = isize;

    fn min() -> Self {
        usize::MIN
    }
    fn max() -> Self {
        usize::MAX
    }

    fn to_step(self) -> Self::Step {
        unsafe { mem::transmute(self) }
    }
    fn from_step(step: Self::Step) -> Self {
        unsafe { mem::transmute(step) }
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        unsafe { mem::transmute(self) }
    }
    fn from_extended_step(step: Self::ExtendedStep) -> Self {
        unsafe { mem::transmute(step) }
    }

    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for isize {
    type Step = isize;
    type ExtendedStep = isize;

    fn min() -> isize {
        isize::MIN
    }
    fn max() -> isize {
        isize::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        extended_step
    }
    
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }
}

impl IteratorOps for f32 {
    type Step = f32;
    type ExtendedStep = f64;

    fn min() -> f32 {
        f32::MIN
    }
    fn max() -> f32 {
        f32::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self as f64
    }
    fn from_extended_step(extended_step: Self::ExtendedStep) -> Self {
        if let Some(result) = f32::from_f64(extended_step) {
            result
        } else {
            0.0
        }
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }

    fn next(&mut self, step: Self::Step) {
        *self += step;
    }
}

impl IteratorOps for f64 {
    type Step = f64;
    type ExtendedStep = f64;

    fn min() -> f64 {
        f64::MIN
    }
    fn max() -> f64 {
        f64::MAX
    }

    fn to_step(self) -> Self::Step {
        self
    }
    fn from_step(step: Self::Step) -> Self {
        step
    }

    fn to_extended_step(self) -> Self::ExtendedStep {
        self
    }
    fn from_extended_step(step: Self::Step) -> Self {
        step
    }
    fn extend_step(step: Self::Step) -> Self::ExtendedStep {
        step as Self::ExtendedStep
    }

    fn next(&mut self, step: Self::Step) {
        *self += step;
    }
}

// impl IteratorOps for f128 {
//     type Step = f128;
//     type ExtendedStep = f128;

//     fn to_step(self) -> Self::Step {
//         self
//     }

//     fn from_step(step: Self::Step) -> Self {
//         step
//     }
// }

pub struct BasicRange<T>
where
    T: IteratorOps,
{
    pub start: T,
    pub end: T,
    pub step: T::Step,

    pub inclusive_or_not_on_step: bool,
    pub invalid_range: bool,
}

impl<T> BasicRange<T>
where
    T: IteratorOps
{
    pub fn new(start: T, mut end: T, step: T::Step, inclusive: bool) -> Self {
        if step == T::Step::zero() {
            panic!("Step can't be 0");
        }

        let mut on_step = true;
        let invalid_range = (start < end && step < T::Step::zero()) ||
            (start > end && step > T::Step::zero());

        if !invalid_range {
            let range_size: T::ExtendedStep = if step > T::Step::zero() {
                end.to_extended_step() - start.to_extended_step()
            } else {
                start.to_extended_step() - end.to_extended_step()
            };

            if start != end {
                if step < T::Step::negative_one() || T::Step::one() < step {
                    (end, on_step) = Self::calculate_stop_and_steps(start, end, range_size, step);
                    if DEBUG_PRINT {
                        println!("end is {}, zero is {}, step is {}, on_step is {}", end, T::Step::zero(), step, on_step);
                    }
                } else {
                    on_step = true;
                }
            }

            if inclusive || !on_step {
                if DEBUG_PRINT {
                    println!("end is {}, zero is {}, step is {}, on_step is {}", end, T::Step::zero(), step, on_step);
                }
                end.next(step);
                if DEBUG_PRINT {
                    println!("end is {}, zero is {}, step is {}, on_step is {}", end, T::Step::zero(), step, on_step);
                }
            }
        }
        if DEBUG_PRINT {
            println!("invalid_range: {}, end: {}, step: {}, inclusive: {}, on_step: {}", invalid_range, end, step, inclusive, on_step);
        }
        BasicRange { start, end, step, inclusive_or_not_on_step: inclusive || !on_step, invalid_range }
    }

    fn calculate_stop_and_steps(start: T, end: T, range_size: T::ExtendedStep, step: T::Step) -> (T, bool)
    where
        T: IteratorOps
    {
        if DEBUG_PRINT {
            println!("start {} end {} range_size {} step {}", start, end, range_size, step);
        }
        let range_size_as_extended_step = range_size;
        let positive_step :T::Step = if step < T::Step::zero() { T::Step::zero() - step } else { step};
        let steps = (range_size_as_extended_step / T::from_step(positive_step).to_extended_step()).floor();
        if DEBUG_PRINT {
            println!("range_size_as_extended_step {} steps {}", range_size_as_extended_step, steps);
        }
        let on_step = T::ExtendedStep::zero() == range_size_as_extended_step.rem(T::extend_step(positive_step));
        let new_range_size = steps * T::from_step(positive_step).to_extended_step();
        let new_end : T::ExtendedStep = start.to_extended_step() + if start < end {
            new_range_size
        } else {
            T::ExtendedStep::zero() - new_range_size
        };
        if DEBUG_PRINT {
            println!("new end {}", new_end);
        }
        (T::from_extended_step(new_end), on_step)
    }
}

pub struct BasicRangeIter<T>
where
    T: IteratorOps,
{
    pub current: T,
    pub end: T,
    pub step: T::Step,

    pub inclusive_or_not_on_step: bool,
    pub invalid_range: bool,
}

impl<T> Iterator for BasicRangeIter<T>
where
    T: IteratorOps,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if DEBUG_PRINT {
            println!("Current {}, step {}, end {}", self.current, self.step, self.end);
        }
        if self.invalid_range {
            None
        } else {
            if DEBUG_PRINT {
                println!("Current {}, step {}, end {} inclusive or not on step {}", self.current, self.step, self.end, self.inclusive_or_not_on_step);
            }
            let stop = self.current == self.end;

            if !self.inclusive_or_not_on_step {
                if stop {
                    if DEBUG_PRINT {
                        println!("Stop!");
                    }
                    return None
                }
            } else {
                self.inclusive_or_not_on_step = false;
            }

            // Zig lesson
            // const result = @addWithOverflow(@as(SignedT, @bitCast(self.curr)), self.step);
            // self.curr = @as(T, @bitCast(result[0]));

            let result = self.current;
            self.current = T::from_extended_step(self.current.to_extended_step() + T::extend_step(self.step));
            if DEBUG_PRINT {
                println!("Current {}, step {}, end {}, current + step {}/{}", result, self.step, self.end, result.to_extended_step() + T::extend_step(self.step), self.current);
            }
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

            inclusive_or_not_on_step: self.inclusive_or_not_on_step,
            invalid_range: self.invalid_range,
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
        T: IteratorOps,
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
        T: IteratorOps,
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
                let expect_none = vec![];
                let expect_once = vec![0];
                verify_range(if inclusive > 0 { expect_once } else { expect_none }, BasicRange::new(0, 0, -2, inclusive > 0));

                let expect_none = vec![];
                let expect_once = vec![0];
                verify_range(if inclusive > 0 { expect_once } else { expect_none }, BasicRange::new(0, 0, 2, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(3, 0, 1, inclusive > 0));

                let expect = vec![];
                verify_range(expect, BasicRange::new(0, 1, -1, inclusive > 0));
            }
        }
        #[test]
        fn void_range_prop_float() {
            for inclusive in 0..=1 {
                let expect_none = vec![];
                let expect_once = vec![0.0];
                verify_range(if inclusive > 0 { expect_once } else { expect_none }, BasicRange::new(0.0, 0.0, -2.0, inclusive > 0));

                let expect_none = vec![];
                let expect_once = vec![0.0];
                verify_range(if inclusive > 0 { expect_once } else { expect_none }, BasicRange::new(0.0, 0.0, 2.0, inclusive > 0));

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
        #[should_panic]
        fn zero_step() {
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