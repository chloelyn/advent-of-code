use std::{collections::HashMap, error::Error, ops::Deref, str::FromStr};

pub fn input() -> &'static str {
    include_str!("../../input/day18.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day18.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let droplet = LavaDroplet::from_str(input).unwrap();
    (droplet.surface_area(), 0)
}

// 4176 -> too high
struct LavaDroplet(HashMap<(isize, isize, isize), bool>);

impl LavaDroplet {
    fn neighbors() -> [(isize, isize, isize); 6] {
        [
            // (x, y, z)
            (0, 1, 0),  // up
            (0, -1, 0), // down
            (-1, 0, 0), // left
            (1, 0, 0),  // right
            (0, 0, 1),  // forward
            (0, 0, -1), // backward
        ]
    }

    fn containment(&self, cube: &(isize, isize, isize)) -> usize {
        Self::neighbors()
            .iter()
            .filter(|(dx, dy, dz)| {
                let x = cube.0 + dx;
                let y = cube.1 + dy;
                let z = cube.2 + dz;
                self.get(&(x, y, z)).is_none()
            })
            .count()
    }

    fn surface_area(&self) -> usize {
        self.keys()
            .map(|cube| self.containment(cube))
            .sum::<usize>()
    }
}

impl Deref for LavaDroplet {
    type Target = HashMap<(isize, isize, isize), bool>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for LavaDroplet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid: HashMap<(isize, isize, isize), bool> = HashMap::new();
        for line in s.lines() {
            let mut nums = line.split(',').map(|item| item.parse::<isize>().unwrap());
            grid.insert((next(&mut nums), next(&mut nums), next(&mut nums)), true);
        }
        Ok(Self(grid))
    }
}

fn next(it: &mut impl Iterator<Item = isize>) -> isize {
    it.next().unwrap()
}

common::test!(day18, (64, 58), (4500, 0));
