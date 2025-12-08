fn solve(banks: &str, digits: usize) -> u32 {
    let mut total = 0;

    for line in banks.lines() {
        let pos1 = line
            .chars()
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .rev()
            .skip(digits - 1)
            .max_by_key(|&(_, c)| c.to_digit(10).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        let pos2 = line
            .chars()
            .enumerate()
            .skip(pos1 + 1)
            .max_by_key(|&(_, c)| c.to_digit(10).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        let digit1 = line.chars().nth(pos1).unwrap().to_digit(10).unwrap();
        let digit2 = line.chars().nth(pos2).unwrap().to_digit(10).unwrap();
        total += digit1 * 10 + digit2;
    }

    total
}

fn main() {
    let banks = std::fs::read_to_string("src/input/2025-03.txt").unwrap();

    println!("{}", solve(&banks, 2));
    // println!("{}", part_two(&banks));
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

// #[test]
// fn test_part_two() {
//     assert_eq!(part_two(&parse("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124")), 4174379265);
// }
