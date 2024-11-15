use crate::basic_range::*;

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
    for n in (start..=end) {
        assert!(start <= n);
        assert!(n <= end);
        index += 1;
    }
    assert_eq!(index, 1 + end as usize - start as usize);

    const mini8:i8 = -128;
    const maxi8:i8 = 127;
    index = 0;
    for i in (mini8..=maxi8) {
        assert!(mini8 <= i);
        assert!(i <= maxi8);
        index += 1;
    }
    // Using extended signed type to calculate, avoiding overflow
    assert_eq!(index, (1 + maxi8 as i16 - mini8 as i16) as usize);
}

}
