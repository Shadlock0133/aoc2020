use std::ops::Index;

const SLOPES: &[(usize, usize)] = &[
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
];

fn main() {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    let map = parse_input(&input).unwrap();
    let res = check(&map, (3, 1));
    println!("Part 1 - Answer: {}", res);
    let res: usize = SLOPES.iter().map(|&s| check(&map, s)).product();
    println!("Part 2 - Answer: {}", res);
}

struct Map {
    map: Vec<bool>,
    width: usize,
}

impl Index<(usize, usize)> for Map {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.map[(y * self.width + x % self.width)]
    }
}

impl Map {
    fn height(&self) -> usize {
        self.map.len() / self.width
    }
}

fn parse_input(input: &str) -> Result<Map, Box<dyn std::error::Error>> {
    let width = input.lines().next().unwrap().trim().chars().count();
    let map: Vec<_> = input
        .chars()
        .filter_map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            c if c.is_whitespace() => None,
            c => unreachable!("got {}", c),
        })
        .collect();
    assert_eq!(map.len() % width, 0, "map must have full last line");
    Ok(Map { map, width })
}

fn check(map: &Map, step: (usize, usize)) -> usize {
    let mut pos = (0, 0);
    let mut trees_hit = 0;
    while pos.1 < map.height() {
        if map[pos] {
            trees_hit += 1;
        }
        pos = (pos.0 + step.0, pos.1 + step.1);
    }
    trees_hit
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = 
    "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#";

    #[test]
    fn test1() {
        let output = 7;
        let res = check(&parse_input(INPUT).unwrap(), (3, 1));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = [2, 7, 3, 4, 2];
        let map = parse_input(INPUT).unwrap();
        let res: Vec<_> = SLOPES.iter().map(|&s| check(&map, s)).collect();
        assert_eq!(res, output);
        let product: usize = res.iter().product();
        assert_eq!(product, 336);
    }
}
