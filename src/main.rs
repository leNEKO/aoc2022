use std::{
    fs::File,
    io::{self, BufRead, Result, Lines, BufReader},
};

fn main() ->Result<()> {
    dbg!(get_max("data/input.txt")?);

    Ok(())
}

fn get_max(filepath: &str) -> Result<u32>{
    let line_iterator = read_lines::<String>(filepath)?;

    let mut max_total = 0;
    let mut current_total = 0;

    for line in line_iterator {
        match line?.parse::<u32>() {
            Ok(value) => current_total += value,
            _ => {
                if current_total > max_total {
                    max_total = current_total;
                }
                current_total = 0;
            }
        }
    };

    Ok(max_total)
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
        get_max("data/test.txt").unwrap()
    );
}
