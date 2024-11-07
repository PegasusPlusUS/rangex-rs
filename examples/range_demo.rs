use rangex::basic_range::*;
use rangex::range_inclusive;

/// DocTest
/// ```
/// let mut s = 0;
/// for v in range_inclusive!(u8, 1, 100, 1) {
///   s += v;
/// }
/// println!("Sum(1..=100):{}", s);
/// assert_eq!(s, 5050);
/// ```
fn main() {
    print!("Calculate by for v in (1..=100):");
    //println!(r"By 'let mut s = 0; for v in range_inclusive!(u8, 1, 100, 1) \{ s += v }'");
    let r = range_inclusive!(u8, 1, 100, 1);
    let mut s: u16 = 0;
    for v in r {
        s += v as u16
    }
    println!("{}", s);

    print!("Calculate by for_each:");
    let r = range_inclusive!(u8, 1, 100, 1);
    s = 0;
    r.into_iter().for_each(|v| s += v as u16);
    println!("{}", s);

    print!("Calculate by map and sum:");
    let r = range_inclusive!(u8, 1, 100, 1);
    s = r.into_iter().map(|x| x as u16).sum();
    println!("{}", s);

    println!();
    println!("Demo to print zigzaged numbers of 5 rows:");
    print_zigzag_to_write(5);
}

fn print_zigzag_to_write(rows: u8) {
    use std::io::Write;
    fn print_zigzag(row: u8, mut writer: impl Write) {
        let mut row_step = 1;
        for r in range_inclusive!(u8, 1, row, 1) {
            let mut col_step = row_step;
            for c in 1..=row - r + 1 {
                let _ = write!(writer, "{:<3}", col_step);
                col_step += r + c;
            }
            let _ = writeln!(writer);
            row_step += r
        }
    }
    print_zigzag(rows, &mut std::io::stdout());
}
