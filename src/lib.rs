//! This crate provides clear range expression for exclusive/inclusive, forward/backward, and step great than 1 or less than -1, with index support.
//!
//! The most convenient way is to use the following macros:
//!
//! **range_inclusive!(*start*, *stop*)**
//!
//! This is the same as ***start*..=*stop***
//!
//!
//! **range_exclusive!(*start*, *stop*)**
//!
//! This is the same as ***start*..*stop***
//!
//!
//! **range_inclusive!(*type*, *start*, *stop*)**
//!
//! Creates range for *type* from *start* **through** *stop*, by **step 1**
//!
//!
//! **range_inclusive!(*type*, *start*, *stop*, *step*)**
//!
//! Creates range for *type* from *start* **through** *stop*, by *step*
//!
//!
//! **indexed_range_inclusive!(*type*, *start*, *stop*)**
//!
//! Creates indexed range for *type* from *start* **through** *stop*, by **step 1**
//!
//!
//! **indexed_range_inclusive!(*type*, *start*, *stop*, *step*)**
//!
//! Creates indexed range for *type* from *start* **through** *stop*, by *step*
//
/// DocTest for basic_range
/// ```
/// use rangex::basic_range::*;
/// use rangex::range_inclusive;
/// let mut s = 0;
/// // create inclusive range of u8 from 1 through 100, default step 1
/// for v in range_inclusive!(u8, 1, 100) {
///   s += v as u16;
/// }
/// println!("Sum(1..=100):{}", s);
/// assert_eq!(s, 5050);
/// ```
pub mod basic_range;
/// DocTest for indexed_range
/// ```
/// use rangex::indexed_range::*;
/// use rangex::indexed_range_inclusive;
/// let mut si = 0;
/// let mut sv = 0;
/// // create indexed inclusive range of u8 from 1 through 100, step 1
/// for (i, v) in indexed_range_inclusive!(u8, 1, 100, 1) {
///   si += i;
///   sv += v as u16;
/// }
/// println!("Sum(1..=100):{}", sv);
/// // index from 0 through 99
/// assert_eq!(si + 100, sv as usize);
/// assert_eq!(sv, 5050);
/// ```
pub mod indexed_range;
