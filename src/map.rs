use std::ops::Index;

pub struct Map2D {
    map: Vec<bool>,
    width: usize,
}

impl Index<(usize, usize)> for Map2D {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        assert!(x < self.width && y < self.height());
        &self.map[(y * self.width + x)]
    }
}

impl Map2D {
    fn height(&self) -> usize {
        self.map.len() / self.width
    }

    pub fn from_str(input: &str) -> Self {
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
        Map2D { map, width }
    }
}