/// DocTest for basic_range
/// ```
/// use rangex::basic_range::*;
/// use rangex::range_inclusive;
/// let mut s = 0;
/// for v in range_inclusive!(u8, 1, 100, 1) {
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
/// for (i, v) in indexed_range_inclusive!(u8, 1, 100, 1) {
///   si += i;
///   sv += v as u16;
/// }
/// println!("Sum(1..=100):{}", sv);
/// assert_eq!(si, 4950);
/// assert_eq!(sv, 5050);
/// ```
pub mod indexed_range;
