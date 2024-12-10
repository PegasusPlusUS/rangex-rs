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

        const MINI8: i8 = -128;
        const MAXI8: i8 = 127;
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

    fn int_edge_with_step<T: crate::basic_range::IteratorOps>(inclusive: bool, step: T::Step) {
        let int_min: T = T::min();
        let int_max: T = T::max();
        let range_size: T::ExtendedStep = int_max.to_extended_step() - int_min.to_extended_step();
        let (steps, on_step): (T::ExtendedStep, bool) = if step > (step - step) {
            (
                (range_size / T::extend_step(step)).floor(),
                range_size.rem(T::extend_step(step)) == (range_size - range_size),
            )
        } else {
            (
                (range_size / (T::extend_step(step) - T::extend_step(step) - T::extend_step(step)))
                    .floor(),
                range_size.rem(T::extend_step(step) - T::extend_step(step) - T::extend_step(step))
                    == (range_size - range_size),
            )
        };
        const DEBUG_PRINT: bool = true;
        // positive step for countup
        // negative step for countdown
        // if step is T::Step::min(), can only do countdown
        if step != T::Step::min() {
            let positive_step = if step > (step - step) {
                step
            } else {
                step - step - step
            };
            // countup
            print!(
                "{} while range [{}, {}{}, step {}",
                type_name::<T>(),
                int_min,
                int_max,
                get_range_end_mark_char(inclusive),
                positive_step
            );
            if DEBUG_PRINT {
                println!(
                    " range_size {}, steps {}, on_step {}:",
                    range_size, steps, on_step
                );
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
                    if i == steps.to_usize() {
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
        if true {
            let negative_step = if step < (step - step) {
                step
            } else {
                step - step - step
            };

            print!(
                "Backward {} while range {}{}, {}], step: {}:\n",
                type_name::<T>(),
                get_range_begin_mark_char(inclusive),
                int_min,
                int_max,
                negative_step
            );
            let range = IndexedRange::<T>::new(int_max, int_min, negative_step, inclusive);
            let mut index: usize = 0;
            for (i, _) in range {
                assert_eq!(i, index);
                index += 1;
                if inclusive || !on_step {
                    assert!(i <= steps.to_usize() + 2);
                } else {
                    if i == steps.to_usize() {
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
    fn test_i8_exclusive_edge_min_step() {
        int_edge_with_step::<i8>(false, i8::MIN);
    }

    #[test]
    fn test_i8_exclusive_edge_max_step() {
        int_edge_with_step::<i8>(false, i8::MAX);
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
    fn test_i8_inclusive_edge_min_step() {
        int_edge_with_step::<i8>(true, i8::MIN);
    }

    #[test]
    fn test_i8_inclusive_edge_max_step() {
        int_edge_with_step::<i8>(true, i8::MAX);
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
    fn test_u8_exclusive_edge_min_step() {
        int_edge_with_step::<u8>(false, i8::MIN);
    }

    #[test]
    fn test_u8_exclusive_edge_max_step() {
        int_edge_with_step::<u8>(false, i8::MAX);
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
    fn test_u8_inclusive_edge_min_step() {
        int_edge_with_step::<u8>(true, i8::MIN);
    }

    #[test]
    fn test_u8_inclusive_edge_max_step() {
        int_edge_with_step::<u8>(true, i8::MAX);
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
    fn test_i16_exclusive_edge_min_step() {
        int_edge_with_step::<i16>(false, i16::MIN);
    }

    #[test]
    fn test_i16_exclusive_edge_max_step() {
        int_edge_with_step::<i16>(false, i16::MAX);
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
    fn test_i16_inclusive_edge_min_step() {
        int_edge_with_step::<i16>(true, i16::MIN);
    }

    #[test]
    fn test_i16_inclusive_edge_max_step() {
        int_edge_with_step::<i16>(true, i16::MAX);
    }

    #[test]
    fn test_u16_exclusive_edge() {
        int_edge_with_step::<u16>(false, 1);
    }

    #[test]
    fn test_u16_exclusive_edge_not_on_step() {
        int_edge_with_step::<u16>(false, 3);
    }

    #[test]
    fn test_u16_exclusive_edge_on_step() {
        int_edge_with_step::<u16>(false, 5);
    }

    #[test]
    fn test_u16_exclusive_edge_min_step() {
        int_edge_with_step::<u16>(false, i16::MIN);
    }

    #[test]
    fn test_u16_exclusive_edge_max_step() {
        int_edge_with_step::<u16>(false, i16::MAX);
    }

    #[test]
    fn test_u16_inclusive_edge() {
        int_edge_with_step::<u16>(true, 1);
    }

    #[test]
    fn test_u16_inclusive_edge_not_on_step() {
        int_edge_with_step::<u16>(true, 3);
    }

    #[test]
    fn test_u16_inclusive_edge_on_step() {
        int_edge_with_step::<u16>(true, 5);
    }

    #[test]
    fn test_u16_inclusive_edge_min_step() {
        int_edge_with_step::<u16>(true, i16::MIN);
    }

    #[test]
    fn test_u16_inclusive_edge_max_step() {
        int_edge_with_step::<u16>(true, i16::MAX);
    }

    #[test]
    fn test_i32_exclusive_edge() {
        int_edge_with_step::<i32>(false, 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_i32_exclusive_edge_not_on_step() {
        int_edge_with_step::<i32>(false, 31 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_i32_exclusive_edge_on_step() {
        int_edge_with_step::<i32>(false, 5 + 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_i32_exclusive_edge_min_step() {
        int_edge_with_step::<i32>(false, i32::MIN);
    }

    #[test]
    fn test_i32_exclusive_edge_max_step() {
        int_edge_with_step::<i32>(false, i32::MAX);
    }

    #[test]
    fn test_i32_inclusive_edge() {
        int_edge_with_step::<i32>(true, 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_i32_inclusive_edge_not_on_step() {
        int_edge_with_step::<i32>(true, 31 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_i32_inclusive_edge_on_step() {
        int_edge_with_step::<i32>(true, 5 + 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_i32_inclusive_edge_min_step() {
        int_edge_with_step::<i32>(true, i32::MIN);
    }

    #[test]
    fn test_i32_inclusive_edge_max_step() {
        int_edge_with_step::<i32>(true, i32::MAX);
    }

    #[test]
    fn test_u32_exclusive_edge() {
        int_edge_with_step::<u32>(false, 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_u32_exclusive_edge_not_on_step() {
        int_edge_with_step::<u32>(false, 31 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_u32_exclusive_edge_on_step() {
        int_edge_with_step::<u32>(false, 5 + 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_u32_exclusive_edge_min_step() {
        int_edge_with_step::<u32>(false, i32::MIN);
    }

    #[test]
    fn test_u32_exclusive_edge_max_step() {
        int_edge_with_step::<u32>(false, i32::MAX);
    }

    #[test]
    fn test_u32_inclusive_edge() {
        int_edge_with_step::<u32>(true, 1 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_u32_inclusive_edge_not_on_step() {
        int_edge_with_step::<u32>(true, 31 + i16::MAX as i32 - i16::MIN as i32);
    }

    #[test]
    fn test_u32_inclusive_edge_on_step() {
        int_edge_with_step::<u32>(true, 5 + 1 + i16::MAX as i32 - i16::MIN as i32);
    }
}
