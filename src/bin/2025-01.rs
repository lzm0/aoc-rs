use std::fs;

enum Direction {
    Left,
    Right,
}

struct Rotation {
    direction: Direction,
    distance: i16,
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|s| {
            let (dir, dist) = s.split_at(1);
            let direction = match dir {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => unreachable!(),
            };
            let distance = dist.parse().unwrap();
            Rotation {
                direction,
                distance,
            }
        })
        .collect()
}

fn part_one(start: i16, rotations: &Vec<Rotation>) -> i16 {
    let mut password = 0;
    let mut position = start;
    for rotation in rotations {
        match rotation.direction {
            Direction::Left => position = position - rotation.distance,
            Direction::Right => position = position + rotation.distance,
        }
        position = position.rem_euclid(100);
        if position == 0 {
            password += 1;
        }
    }
    password
}

fn part_two(start: i16, rotations: &Vec<Rotation>) -> i16 {
    let mut password = 0;
    let mut position = start;
    for rotation in rotations {
        let new_position = match rotation.direction {
            Direction::Left => position - rotation.distance,
            Direction::Right => position + rotation.distance,
        };
        if position != 0 && (new_position < 1 || new_position > 99) {
            password += 1
        }
        position = new_position.rem_euclid(100);
    }
    password
}

fn main() {
    let lines: Vec<Rotation> = parse_input(
        fs::read_to_string("src/input/2025-01.txt")
            .unwrap()
            .as_str(),
    );

    println!("{}", part_one(50, &lines));
    println!("{}", part_two(50, &lines));
}

#[test]
fn test() {
    use indoc::indoc;

    let input = parse_input(indoc! {"
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
    assert_eq!(part_one(50, &input), 3);
    assert_eq!(part_two(50, &input), 6);
}
