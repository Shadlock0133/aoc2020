fn main() {
    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    let list = parse_input(&input);
    let res = check_1(&list);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&list);
    println!("Part 2 - Answer: {}", res);
}

enum Dir {
    North,
    South,
    East,
    West,
}

enum Move {
    Dir(Dir),
    Left,
    Right,
    Forward,
}

impl From<char> for Move {
    fn from(ch: char) -> Self {
        match ch {
            'N' => Move::Dir(Dir::North),
            'S' => Move::Dir(Dir::South),
            'E' => Move::Dir(Dir::East),
            'W' => Move::Dir(Dir::West),
            'L' => Move::Left,
            'R' => Move::Right,
            'F' => Move::Forward,
            _ => unreachable!(),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Move, u32)> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line.trim();
            let m = line.chars().next().unwrap().into();
            let n = line[1..].parse().unwrap();
            (m, n)
        })
        .collect()
}

fn check_1(list: &[(Move, u32)]) -> u32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut dir = Dir::East;

    for (m, n) in list {
        match m {
            Move::Dir(Dir::North) => pos.1 += *n as i32,
            Move::Dir(Dir::South) => pos.1 -= *n as i32,
            Move::Dir(Dir::East) => pos.0 += *n as i32,
            Move::Dir(Dir::West) => pos.0 -= *n as i32,
            Move::Forward if matches!(dir, Dir::North) => pos.1 += *n as i32,
            Move::Forward if matches!(dir, Dir::South) => pos.1 -= *n as i32,
            Move::Forward if matches!(dir, Dir::East) => pos.0 += *n as i32,
            Move::Forward if matches!(dir, Dir::West) => pos.0 -= *n as i32,
            Move::Forward => unreachable!(),
            Move::Left => {
                for _ in 0..(n / 90) {
                    dir = match dir {
                        Dir::North => Dir::West,
                        Dir::South => Dir::East,
                        Dir::East => Dir::North,
                        Dir::West => Dir::South,
                    }
                }
            }
            Move::Right => {
                for _ in 0..(n / 90) {
                    dir = match dir {
                        Dir::North => Dir::East,
                        Dir::South => Dir::West,
                        Dir::East => Dir::South,
                        Dir::West => Dir::North,
                    }
                }
            }
        }
    }

    pos.0.abs() as u32 + pos.1.abs() as u32
}

fn check_2(list: &[(Move, u32)]) -> u32 {
    let mut waypoint: (i32, i32) = (10, 1);
    let mut pos: (i32, i32) = (0, 0);

    for (m, n) in list {
        match m {
            Move::Dir(Dir::North) => waypoint.1 += *n as i32,
            Move::Dir(Dir::South) => waypoint.1 -= *n as i32,
            Move::Dir(Dir::East) => waypoint.0 += *n as i32,
            Move::Dir(Dir::West) => waypoint.0 -= *n as i32,
            Move::Forward => {
                pos.0 = pos.0 + *n as i32 * waypoint.0;
                pos.1 = pos.1 + *n as i32 * waypoint.1;
            }
            Move::Left => {
                for _ in 0..(n / 90) {
                    waypoint = (-waypoint.1, waypoint.0);
                }
            }
            Move::Right => {
                for _ in 0..(n / 90) {
                    waypoint = (waypoint.1, -waypoint.0);
                }
            }
        }
    }

    pos.0.abs() as u32 + pos.1.abs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "F10
        N3
        F7
        R90
        F11";

    #[test]
    fn test1() {
        let output = 25;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 286;
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
