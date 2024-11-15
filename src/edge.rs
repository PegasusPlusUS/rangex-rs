use crate::basic_range::*;
use crate::indexed_range::*;

#[cfg(test)]
mod edge_test {

#[test]
fn std_u8_exclusive_edge_can_through_full_u8() {
    // std for in zig only allow usize
    // Loop from -128 to 127 exclusive
    //for (std.math.minInt(i8)..std.math.maxInt(i8)) |i| {
    // Do something with `i`
    //    std.debug.print("i: {}\n", .{i});
    //}
    const start: u8 = 0;
    const end: u8 = 255;
    let mut index: usize = 0;
    for n in start..=end {
        assert!(start <= n);
        assert!(n <= end);
        index += 1;
    }
    assert_eq!(index, 1 + end as usize - start as usize);

    const mini8:i8 = -128;
    const maxi8:i8 = 127;
    index = 0;
    for i in mini8..=maxi8 {
        assert!(mini8 <= i);
        assert!(i <= maxi8);
        index += 1;
    }
    // Using extended signed type to calculate, avoiding overflow
    assert_eq!(index, (1 + maxi8 as i16 - mini8 as i16) as usize);
}

fn get_range_end_mark_char(inclusive: bool) -> char {
    if inclusive {
        return ']';
    } else {
        return ')';
    }
}

fn get_range_begin_mark_char(inclusive: bool) -> char {
    if inclusive {
        return '[';
    } else {
        return '(';
    }
}

use crate::basic_range::*;
use crate::indexed_range::*;
use std::any::type_name;
use std::ops::Rem;
use num::zero;

fn intEdgeWithStep<T: crate::basic_range::IteratorOps>(inclusive: bool, step: T::Step) {
    let intMin: T = T::min();
    let intMax: T = T::max();
    let range_size: T::ExtendedStep = intMax.to_extended_step() - intMin.to_extended_step();
    let steps: T::ExtendedStep = (range_size / T::from_step(step).to_extended_step()).floor();
    let on_step = range_size.rem(T::from_step(step).to_extended_step()) == (steps - steps);
    let debug_print = false;
    if true
    {
        print!("{} while range [{}, {}{}, step {}", type_name::<T>(), intMin, intMax, get_range_end_mark_char(inclusive), step);
        if debug_print {
            println!(" range_size {}, steps {}, on_step {}:", range_size, steps, on_step);
        }
        println!("");
        let mut range = IndexedRange::<T>::new(intMin, intMax, step, inclusive);
        let mut index: usize = 0;
        for (i, _) in range {
            assert_eq!(i, index);
            index += 1;
            if inclusive || !on_step {
                assert!(i <= T::from_extended_step(steps).to_usize());
            } else {
                assert!(i < T::from_extended_step(steps).to_usize());
            }
        }

        if inclusive || !on_step {
            assert_eq!(T::from_extended_step(steps).to_usize() + 1, index);
        } else {
            assert_eq!(T::from_extended_step(steps).to_usize(), index);
        }
    }

    // Backward
    if true
    {
        print!("Backward {} while range {}{}, {}], step: -{}:\n", type_name::<T>(), get_range_begin_mark_char(inclusive), intMin, intMax, step);
        let mut range = IndexedRange::<T>::new(intMax, intMin, zero::<T::Step>() - step, inclusive);
        let mut index: usize = 0;
        for (i, _) in range {
            assert_eq!(i, index);
            index += 1;
            if inclusive || !on_step {
                assert!(i <= T::from_extended_step(steps).to_usize());
            } else {
                assert!(i < T::from_extended_step(steps).to_usize());
            }
        }

        if inclusive || !on_step {
            assert_eq!(T::from_extended_step(steps).to_usize() + 1, index);
        } else {
            assert_eq!(T::from_extended_step(steps).to_usize(), index);
        }
    }
}

#[test]
//#[should_panic]
fn test_i8_exclusive_edge() {
    intEdgeWithStep::<i8>(false, 1);
}

#[test]
#[should_panic]
fn test_i8_inclusive_edge() {
    intEdgeWithStep::<u8>(true, 1);
}


}
