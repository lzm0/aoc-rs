fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .map(|line| {
            let (first, last) = line.split_once('-').unwrap();
            (first.parse().unwrap(), last.parse().unwrap())
        })
        .collect()
}

fn part_one(ranges: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    for &(first, last) in ranges {
        for num in first..=last {
            let str = num.to_string();
            let digits = str.len();
            let mid = digits / 2;
            if digits % 2 == 0 && str[..mid] == str[mid..] {
                sum += num;
            }
        }
    }
    sum
}

fn part_two(ranges: &[(u64, u64)]) -> u64 {
    let mut sum = 0;
    for &(first, last) in ranges {
        for num in first..=last {
            let str = num.to_string();
            let digits = str.len();
            for splits in 2..=digits {
                if digits % splits != 0 {
                    continue;
                }
                let part_len = digits / splits;
                let part = &str[..part_len];
                if (1..splits).all(|i| &str[i * part_len..(i + 1) * part_len] == part) {
                    sum += num;
                    break;
                }
            }
        }
    }
    sum
}

fn main() {
    let ranges = parse(
        std::fs::read_to_string("src/input/2025-02.txt")
            .unwrap()
            .as_str(),
    );

    println!("{}", part_one(&ranges));
    println!("{}", part_two(&ranges));
}

#[test]
fn test_part_one() {
    assert_eq!(part_one(&parse("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124")), 1227775554);
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(&parse("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124")), 4174379265);
}
