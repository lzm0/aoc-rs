fn solve(banks: &str, digits: usize) -> u64 {
    let mut total: u64 = 0;

    for line in banks.lines() {
        let mut joltage: u64 = 0;
        let mut start: usize = 0;

        for current_digit in 0..digits {
            let end = line.len() - digits + current_digit;
            let (pos, digit) = line[start..=end]
                .chars()
                .enumerate()
                .collect::<Vec<_>>()
                .iter()
                .rev()
                .max_by_key(|&(_, c)| c.to_digit(10).unwrap())
                .map(|(i, c)| (i + start, c.to_digit(10).map(u64::from).unwrap()))
                .unwrap();
            joltage = joltage * 10 + digit;
            start = pos + 1;
        }

        total += joltage;
    }

    total
}

fn main() {
    let banks = std::fs::read_to_string("src/input/2025-03.txt").unwrap();

    println!("{}", solve(&banks, 2));
    println!("{}", solve(&banks, 12));
}

#[test]
fn test_part_one() {
    assert_eq!(
        solve(
            &"987654321111111\n811111111111119\n234234234234278\n818181911112111",
            2
        ),
        357
    );
}

#[test]
fn test_part_two() {
    assert_eq!(
        solve(
            &"987654321111111\n811111111111119\n234234234234278\n818181911112111",
            12
        ),
        3121910778619
    );
}
