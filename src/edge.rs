#[cfg(test)]
mod edge_test {
use crate::basic_range::*;
use crate::indexed_range::*;
    
#[test]
fn std_u8_exclusive_edge_can_through_full_u8() {
    // std for in zig only allow usize
    // Loop from -128 to 127 exclusive
    //for (std.math.minInt(i8)..std.math.maxInt(i8)) |i| {
    // Do something with `i`
    //    std.debug.print("i: {}\n", .{i});
    //}
    const START: u8 = 0;
    const END: u8 = 255;
    let mut index: usize = 0;
    for n in START..=END {
        assert!(START <= n);
        assert!(n <= END);
        index += 1;
    }
    assert_eq!(index, 1 + END as usize - START as usize);

    const MINI8:i8 = -128;
    const MAXI8:i8 = 127;
    index = 0;
    for i in MINI8..=MAXI8 {
        assert!(MINI8 <= i);
        assert!(i <= MAXI8);
        index += 1;
    }
    // Using extended signed type to calculate, avoiding overflow
    assert_eq!(index, (1 + MAXI8 as i16 - MINI8 as i16) as usize);
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

use std::any::type_name;
use std::ops::Rem;
use num::zero;

fn int_edge_with_step<T: crate::basic_range::IteratorOps>(inclusive: bool, step: T::Step) {
    let int_min: T = T::min();
    let int_max: T = T::max();
    let range_size: T::ExtendedStep = int_max.to_extended_step() - int_min.to_extended_step();
    let steps: T::ExtendedStep = (range_size / T::from_step(step).to_extended_step()).floor();
    let on_step = range_size.rem(T::from_step(step).to_extended_step()) == (steps - steps);
    const DEBUG_PRINT: bool = false;
    if true
    {
        print!("{} while range [{}, {}{}, step {}", type_name::<T>(), int_min, int_max, get_range_end_mark_char(inclusive), step);
        if DEBUG_PRINT {
            println!(" range_size {}, steps {}, on_step {}:", range_size, steps, on_step);
        }
        println!("");
        let range = IndexedRange::<T>::new(int_min, int_max, step, inclusive);
        let mut index: usize = 0;
        for (i, _) in range {
            assert_eq!(i, index);
            index += 1;
            if inclusive || !on_step {
                assert!(i <= steps.to_usize());
            } else {
                if i + 2 > steps.to_usize() {
                    println!("i: {}, steps:{}, {}", i, steps, steps.to_usize());
                    
                }
                assert!(i < steps.to_usize());
            }
        }

        if inclusive || !on_step {
            assert_eq!(steps.to_usize() + 1, index);
        } else {
            assert_eq!(steps.to_usize(), index);
        }
    }

    // Backward
    if true
    {
        print!("Backward {} while range {}{}, {}], step: -{}:\n", type_name::<T>(), get_range_begin_mark_char(inclusive), int_min, int_max, step);
        let range = IndexedRange::<T>::new(int_max, int_min, zero::<T::Step>() - step, inclusive);
        let mut index: usize = 0;
        for (i, _) in range {
            assert_eq!(i, index);
            index += 1;
            if inclusive || !on_step {
                assert!(i <= steps.to_usize() + 2);
            } else {
                assert!(i < steps.to_usize());
            }
        }

        if inclusive || !on_step {
            assert_eq!(steps.to_usize() + 1, index);
        } else {
            assert_eq!(steps.to_usize(), index);
        }
    }
}

#[test]
fn test_i8_exclusive_edge() {
    int_edge_with_step::<i8>(false, 1);
}

#[test]
fn test_i8_exclusive_edge_not_on_step() {
    int_edge_with_step::<i8>(false, 3);
}

#[test]
fn test_i8_exclusive_edge_on_step() {
    int_edge_with_step::<i8>(false, 5);
}

#[test]
fn test_i8_inclusive_edge() {
    int_edge_with_step::<i8>(true, 1);
}

#[test]
fn test_i8_inclusive_edge_not_on_step() {
    int_edge_with_step::<i8>(true, 3);
}

#[test]
fn test_i8_inclusive_edge_on_step() {
    int_edge_with_step::<i8>(true, 5);
}

#[test]
fn test_u8_exclusive_edge() {
    int_edge_with_step::<u8>(false, 1);
}

#[test]
fn test_u8_exclusive_edge_not_on_step() {
    int_edge_with_step::<u8>(false, 3);
}

#[test]
fn test_u8_exclusive_edge_on_step() {
    int_edge_with_step::<u8>(false, 5);
}

#[test]
fn test_u8_inclusive_edge() {
    int_edge_with_step::<u8>(true, 1);

}

#[test]
fn test_u8_inclusive_edge_not_on_step() {
    int_edge_with_step::<u8>(true, 3);

}

#[test]
fn test_u8_inclusive_edge_on_step() {
    int_edge_with_step::<u8>(true, 5);

}

#[test]
fn test_i16_exclusive_edge() {
    int_edge_with_step::<i16>(false, 1);
}

#[test]
fn test_i16_exclusive_edge_not_on_step() {
    int_edge_with_step::<i16>(false, 3);
}

#[test]
fn test_i16_exclusive_edge_on_step() {
    int_edge_with_step::<i16>(false, 5);
}

#[test]
fn test_i16_inclusive_edge() {
    int_edge_with_step::<i16>(true, 1);
}

#[test]
fn test_i16_inclusive_edge_not_on_step() {
    int_edge_with_step::<i16>(true, 3);
}

#[test]
fn test_i16_inclusive_edge_on_step() {
    int_edge_with_step::<i16>(true, 5);
}

#[test]
fn test_u16_exclusive_edge() {
    int_edge_with_step::<u16>(false, 1);
}

#[test]
fn test_u16_inclusive_edge() {
    int_edge_with_step::<u16>(true, 1);

}

}
