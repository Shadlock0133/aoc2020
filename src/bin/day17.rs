fn main() {
    let input = std::fs::read_to_string("inputs/day17.txt").unwrap();
    let slice = parse_input(&input);
    let res = check_1(&slice);
    println!("Part 1 - Answer: {}", res);
    let res = check_2(&slice);
    println!("Part 1 - Answer: {}", res);
}

trait Map {
    type Index;
}

#[derive(Clone, Eq, PartialEq)]
struct Map3D {
    maps: Vec<bool>,
    width: usize,
    height: usize,
}

impl Map for Map3D {
    type Index = (usize, usize, usize);
}

impl Map3D {
    fn new(width: usize, height: usize, depth: usize) -> Self {
        Self {
            maps: vec![false; width * height * depth],
            width,
            height,
        }
    }

    fn check_neighbors1(&self, pos: <Self as Map>::Index) -> usize {
        let (x, y, z) = pos;
        let mut occupied = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let neighbor_pos = (
                        (x + i).wrapping_sub(1),
                        (y + j).wrapping_sub(1),
                        (z + k).wrapping_sub(1),
                    );
                    let neighbor = self.get(neighbor_pos);
                    if neighbor_pos != pos && matches!(neighbor, Some(true)) {
                        occupied += 1;
                    }
                }
            }
        }
        occupied
    }

    fn next(&self, f: impl Fn(&Self, <Self as Map>::Index) -> usize) -> Self {
        let mut next = self.clone();
        for z in 0..self.depth() {
            for y in 0..self.height {
                for x in 0..self.width {
                    let occupied = f(&self, (x, y, z));
                    let c = next.get_mut((x, y, z)).unwrap();
                    *c = match (*c, occupied) {
                        (true, 2..=3) | (false, 3) => true,
                        _ => false,
                    }
                }
            }
        }
        next
    }

    fn next1(&self) -> Self {
        self.next(Self::check_neighbors1)
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn depth(&self) -> usize {
        self.maps.len() / self.size()
    }

    fn get(&self, idx: <Self as Map>::Index) -> Option<&bool> {
        let (x, y, z) = idx;
        if x >= self.width || y >= self.height || z >= self.depth() {
            return None;
        }
        let size = self.size();
        Some(&self.maps[z * size + y * self.width + x])
    }

    fn get_mut(&mut self, idx: <Self as Map>::Index) -> Option<&mut bool> {
        let (x, y, z) = idx;
        if x >= self.width || y >= self.height || z >= self.depth() {
            return None;
        }
        let size = self.size();
        Some(&mut self.maps[z * size + y * self.width + x])
    }

    fn taken(&self) -> usize {
        self.maps.iter().filter(|x| **x).count()
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Map4D {
    maps: Vec<bool>,
    width: usize,
    height: usize,
    depth: usize,
}

impl Map for Map4D {
    type Index = (usize, usize, usize, usize);
}

impl Map4D {
    fn new(width: usize, height: usize, depth: usize, dim4: usize) -> Self {
        Self {
            maps: vec![false; width * height * depth * dim4],
            width,
            height,
            depth,
        }
    }

    fn check_neighbors1(&self, pos: <Self as Map>::Index) -> usize {
        let (x, y, z, w) = pos;
        let mut occupied = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        let neighbor_pos = (
                            (x + i).wrapping_sub(1),
                            (y + j).wrapping_sub(1),
                            (z + k).wrapping_sub(1),
                            (w + l).wrapping_sub(1),
                        );
                        let neighbor = self.get(neighbor_pos);
                        if neighbor_pos != pos && matches!(neighbor, Some(true)) {
                            occupied += 1;
                        }
                    }
                }
            }
        }
        occupied
    }

    fn next(&self, f: impl Fn(&Self, <Self as Map>::Index) -> usize) -> Self {
        let mut next = self.clone();
        for w in 0..self.dim4() {
            for z in 0..self.depth {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let occupied = f(&self, (x, y, z, w));
                        let c = next.get_mut((x, y, z, w)).unwrap();
                        *c = match (*c, occupied) {
                            (true, 2..=3) | (false, 3) => true,
                            _ => false,
                        }
                    }
                }
            }
        }
        next
    }

    fn next1(&self) -> Self {
        self.next(Self::check_neighbors1)
    }

    fn size2d(&self) -> usize {
        self.width * self.height
    }
    fn size3d(&self) -> usize {
        self.width * self.height * self.depth
    }

    fn dim4(&self) -> usize {
        self.maps.len() / self.size3d()
    }

    fn get(&self, idx: <Self as Map>::Index) -> Option<&bool> {
        let (x, y, z, w) = idx;
        if x >= self.width || y >= self.height || z >= self.depth || w >= self.dim4() {
            return None;
        }
        let size2d = self.size2d();
        let size3d = self.size3d();
        Some(&self.maps[w * size3d + z * size2d + y * self.width + x])
    }

    fn get_mut(&mut self, idx: <Self as Map>::Index) -> Option<&mut bool> {
        let (x, y, z, w) = idx;
        if x >= self.width || y >= self.height || z >= self.depth || w >= self.dim4() {
            return None;
        }
        let size2d = self.size2d();
        let size3d = self.size3d();
        Some(&mut self.maps[w * size3d + z * size2d + y * self.width + x])
    }

    fn taken(&self) -> usize {
        self.maps.iter().filter(|x| **x).count()
    }
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let lines = input.lines().map(str::trim).filter(|line| !line.is_empty());
    let width = lines.clone().next().unwrap().chars().count();
    let height = lines.clone().count();
    let map = lines
        .map(|line| {
            let line = line
                .chars()
                .map(|ch| match ch {
                    '#' => true,
                    '.' => false,
                    c => panic!("Unexpected character: {}", c),
                })
                .collect::<Vec<_>>();
            assert_eq!(line.len(), width);
            line
        })
        .collect::<Vec<_>>();
    assert_eq!(map.len(), height);
    map
}

fn check_1(input: &[Vec<bool>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let margin = 7;
    let overhead = 2 * margin;
    let mut map = Map3D::new(width + overhead, height + overhead, 1 + overhead);
    for y in 0..width {
        for x in 0..height {
            let pos = (x + margin, y + margin, 1 + margin);
            *map.get_mut(pos).unwrap() = input[y][x];
        }
    }
    for _ in 0..6 {
        map = map.next1();
    }
    map.taken()
}

fn check_2(input: &[Vec<bool>]) -> usize {
    let height = input.len();
    let width = input[0].len();
    let margin = 7;
    let overhead = 2 * margin;
    let mut map = Map4D::new(
        width + overhead,
        height + overhead,
        1 + overhead,
        1 + overhead,
    );
    for y in 0..width {
        for x in 0..height {
            let pos = (x + margin, y + margin, 1 + margin, 1 + margin);
            *map.get_mut(pos).unwrap() = input[y][x];
        }
    }
    for _ in 0..6 {
        map = map.next1();
    }
    map.taken()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".#.
        ..#
        ###";

    #[test]
    fn test1() {
        let output = 112;
        let res = check_1(&parse_input(INPUT));
        assert_eq!(res, output);
    }

    #[test]
    #[ignore = "takes too long"]
    fn test2() {
        let output = 848;
        let res = check_2(&parse_input(INPUT));
        assert_eq!(res, output);
    }
}
