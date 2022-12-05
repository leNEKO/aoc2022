use std::{
    fs::File,
    io::{self, BufRead, Result, Lines, BufReader},
};

fn main() ->Result<()> {
    dbg!(get_max("data/input.txt", 1)?);
    dbg!(get_max("data/input.txt", 3)?);

    Ok(())
}

fn get_max(filepath: &str, top: usize) -> Result<u32>{
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
    };
    totals.push(current_total);
    totals.sort();

    let sum = totals.iter().rev().take(top).sum();
    
    Ok(sum)
}

fn read_lines<T>(filename: &str) -> Result<Lines<BufReader<File>>>{
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    Ok(reader.lines())
}

#[test]
fn test_get_max() {
    assert_eq!(
        24000_u32,
        get_max("data/test.txt", 1).unwrap()
    );
}

#[test]
fn test_get_top3_max() {
    assert_eq!(
        45000,
        get_max("data/test.txt", 3).unwrap()
    );
}
