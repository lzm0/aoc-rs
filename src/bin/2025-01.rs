use std::fs;

fn parse(input: &str) -> Vec<i16> {
    input
        .lines()
        .map(|s| {
            let (dir, dist) = s.split_at(1);
            let distance: i16 = dist.parse().unwrap();
            match dir {
                "L" => -distance,
                "R" => distance,
                _ => unreachable!(),
            }
        })
        .collect()
}

fn part_one(rotations: &[i16]) -> i16 {
    let mut password = 0;
    let mut position = 50;
    for rotation in rotations {
        position += rotation;
        position = position.rem_euclid(100);
        if position == 0 {
            password += 1;
        }
    }
    password
}

fn part_two(rotations: &[i16]) -> i16 {
    let mut password = 0;
    let mut position = 50;
    for &rotation in rotations {
        if rotation >= 0 {
            password += (position + rotation) / 100;
        } else {
            let complement = (100 - position) % 100;
            password += (complement - rotation) / 100;
        }
        position += rotation;
        position = position.rem_euclid(100);
    }
    password
}

fn main() {
    let lines: Vec<i16> = parse(
        fs::read_to_string("src/input/2025-01.txt")
            .unwrap()
            .as_str(),
    );

    println!("{}", part_one(&lines));
    println!("{}", part_two(&lines));
}

#[test]
fn test() {
    use indoc::indoc;

    let input = parse(indoc! {"
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
    "});
    assert_eq!(part_one(&input), 3);
    assert_eq!(part_two(&input), 6);
}
