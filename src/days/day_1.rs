use crate::helpers::read_lines;
use std::io::Result;

pub fn get_max(filepath: &str, top: usize) -> Result<u32> {
    let line_iterator = read_lines::<String>(filepath)?;

    let mut totals: Vec<u32> = vec![];
    let mut current_total = 0;

    for line in line_iterator {
        match line?.parse::<u32>() {
            Ok(value) => current_total += value,
            _ => {
                totals.push(current_total);
                current_total = 0;
            }
        }
    }
    totals.push(current_total);
    totals.sort();

    let sum = totals.iter().rev().take(top).sum();

    Ok(sum)
}

#[test]
fn test_get_max() {
    let top = 1;
    assert_eq!(24000, get_max("data/day_1/test.txt", top).unwrap());
    dbg!(get_max("data/day_1/input.txt", top).unwrap());
}

#[test]
fn test_get_top3_max() {
    let top = 3;
    assert_eq!(45000, get_max("data/day_1/test.txt", top).unwrap());
    dbg!(get_max("data/day_1/input.txt", top).unwrap());
}
