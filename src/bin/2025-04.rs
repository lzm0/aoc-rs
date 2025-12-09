fn part_one(input: &str) -> u32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let m = grid.len();
    let n = grid[0].len();
    let mut result = 0;
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '@' {
                let mut count = 0;
                for &(di, dj) in &[
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    let ni = i as isize + di;
                    let nj = j as isize + dj;
                    if ni >= 0
                        && ni < m as isize
                        && nj >= 0
                        && nj < n as isize
                        && grid[ni as usize][nj as usize] == '@'
                    {
                        count += 1;
                        if count >= 4 {
                            break;
                        }
                    }
                }
                if count < 4 {
                    result += 1;
                }
            }
        }
    }
    result
}

fn part_two(input: &str) -> u32 {}

fn main() {
    let input = std::fs::read_to_string("src/input/2025-04.txt").unwrap();

    println!("{}", part_one(&input));
}

#[test]
fn test_part_one() {
    assert_eq!(
        part_one(indoc::indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
    "}),
        13
    );
}

#[test]
fn test_part_two() {
    assert_eq!(
        part_two(indoc::indoc! {"
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
    "}),
        43
    );
}
