use std::fmt;

fn main() {
    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
    let map = parse_input(&input);
    let res = check_1(&map);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&map);
    println!("Part 2 - Answer: {}", res);
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum State {
    Floor,
    Seat,
    Occupied,
}

#[derive(Clone, Eq, PartialEq)]
struct Map {
    map: Vec<State>,
    width: usize,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        for y in 0..self.height() {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.get((x, y)).unwrap() {
                        State::Floor => '.',
                        State::Seat => 'L',
                        State::Occupied => '#',
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn check_neighbors1(&self, pos: (usize, usize)) -> usize {
        let (x, y) = pos;
        let mut occupied = 0;
        for i in 0..3 {
            for j in 0..3 {
                let neighbor_pos = ((x + j).wrapping_sub(1), (y + i).wrapping_sub(1));
                let neighbor = self.get(neighbor_pos);
                if neighbor_pos != pos && matches!(neighbor, Some(State::Occupied)) {
                    occupied += 1;
                }
            }
        }
        occupied
    }

    fn check_neighbors2(&self, pos: (usize, usize)) -> usize {
        fn find_seat(map: &Map, mut pos: (usize, usize), step: (isize, isize)) -> Option<bool> {
            loop {
                pos.0 = (pos.0 as isize + step.0) as usize;
                pos.1 = (pos.1 as isize + step.1) as usize;
                match map.get(pos)? {
                    State::Seat => return Some(false),
                    State::Occupied => return Some(true),
                    _ => (),
                }
            }
        }
        let mut occupied = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if (i, j) != (0, 0) && matches!(find_seat(&self, pos, (i, j)), Some(true)) {
                    occupied += 1;
                }
            }
        }
        occupied
    }

    fn next(&self, n: usize, f: impl Fn(&Map, (usize, usize)) -> usize) -> Map {
        let mut next = self.clone();
        for y in 0..self.height() {
            for x in 0..self.width {
                let occupied = f(&self, (x, y));
                let c = next.get_mut((x, y)).unwrap();
                match c {
                    State::Seat if occupied == 0 => *c = State::Occupied,
                    State::Occupied if occupied >= n => *c = State::Seat,
                    _ => (),
                }
            }
        }
        next
    }

    fn next1(&self) -> Map {
        self.next(4, Self::check_neighbors1)
    }

    fn next2(&self) -> Map {
        self.next(5, Self::check_neighbors2)
    }

    fn height(&self) -> usize {
        self.map.len() / self.width
    }

    fn get(&self, idx: (usize, usize)) -> Option<&State> {
        let (x, y) = idx;
        if x >= self.width || y >= self.height() {
            return None;
        }
        Some(&self.map[y * self.width + x])
    }

    fn get_mut(&mut self, idx: (usize, usize)) -> Option<&mut State> {
        let (x, y) = idx;
        if x >= self.width || y >= self.height() {
            return None;
        }
        Some(&mut self.map[y * self.width + x])
    }

    fn taken(&self) -> usize {
        self.map
            .iter()
            .filter(|x| matches!(x, State::Occupied))
            .count()
    }
}

fn parse_input(input: &str) -> Map {
    let width = input.lines().next().unwrap().chars().count();
    let map = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let line = line
                .trim()
                .chars()
                .map(|ch| match ch {
                    '.' => State::Floor,
                    'L' => State::Seat,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            assert_eq!(line.len(), width);
            line
        })
        .flatten()
        .collect();
    Map { map, width }
}

fn check_1(map: &Map) -> usize {
    let mut map = map.clone();
    loop {
        let next = map.next1();
        if next == map {
            return map.taken();
        }
        map = next;
    }
}

fn check_2(map: &Map) -> usize {
    let mut map = map.clone();
    loop {
        let next = map.next2();
        if next == map {
            return map.taken();
        }
        map = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "L.LL.LL.LL
        LLLLLLL.LL
        L.L.L..L..
        LLLL.LL.LL
        L.LL.LL.LL
        L.LLLLL.LL
        ..L.L.....
        LLLLLLLLLL
        L.LLLLLL.L
        L.LLLLL.LL";

    #[test]
    fn test1() {
        let output = 37;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    fn test2() {
        let output = 26;
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
