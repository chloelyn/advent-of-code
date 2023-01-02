use std::str::FromStr;

pub fn input() -> &'static str {
    include_str!("../../input/day08.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day08.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let grove = TreeGrove::from_str(input).unwrap();
    let inner = grove.trees.len() - 2;
    let mut count = inner * 4 + 4;
    let mut scenic_scores: Vec<usize> = Vec::with_capacity(grove.trees.len() * grove.trees.len());
    for row in 1..grove.trees.len() - 1 {
        for col in 1..grove.trees.len() - 1 {
            scenic_scores.push(grove.scenic_score((row, col)));
            if grove.visible_from_edge((row, col)) {
                count += 1;
            }
        }
    }
    (count, scenic_scores.into_iter().max().unwrap())
}

const DIRECTIONS: [Direction; 4] = [
    Direction::Left,
    Direction::Right,
    Direction::Up,
    Direction::Down,
];

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct TreeGrove {
    trees: Vec<Vec<u32>>,
}

impl TreeGrove {
    fn visible(&self, (row, col): (usize, usize), direction: &Direction) -> bool {
        let checking = self.trees[row][col];
        match direction {
            Direction::Left => self.trees[row][0..col].iter().all(|&tree| tree < checking),
            Direction::Right => self.trees[row][col + 1..=self.trees[row].len() - 1]
                .iter()
                .all(|&tree| tree < checking),
            Direction::Up => (0..row).all(|row| self.trees[row][col] < checking),
            Direction::Down => {
                (row + 1..=self.trees.len() - 1).all(|row| self.trees[row][col] < checking)
            }
        }
    }

    fn visible_from_edge(&self, (row, col): (usize, usize)) -> bool {
        DIRECTIONS
            .iter()
            .any(|direction| self.visible((row, col), direction))
    }

    fn scenic_score(&self, (row, col): (usize, usize)) -> usize {
        DIRECTIONS
            .iter()
            .map(|direction| self.view_distance((row, col), direction))
            .reduce(|acc, dist| acc * dist)
            .unwrap()
    }

    fn view_distance(&self, (row, col): (usize, usize), direction: &Direction) -> usize {
        match direction {
            Direction::Left => self.find_sum((row, col), (0..col).rev(), false),
            Direction::Right => self.find_sum((row, col), col + 1..=self.trees.len() - 1, false),
            Direction::Up => self.find_sum((row, col), (0..row).rev(), true),
            Direction::Down => self.find_sum((row, col), row + 1..=self.trees.len() - 1, true),
        }
    }

    fn find_sum<I>(&self, (row, col): (usize, usize), values: I, vertical: bool) -> usize
    where
        I: IntoIterator<Item = usize>,
    {
        let checking = self.trees[row][col];
        let mut sum = 0;
        for value in values {
            let tree = if vertical {
                self.trees[value][col]
            } else {
                self.trees[row][value]
            };
            if tree <= checking {
                sum += 1;
                if tree == checking {
                    break;
                }
            } else if tree > checking {
                sum += 1;
                break;
            }
        }
        sum
    }
}

impl FromStr for TreeGrove {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut trees = vec![];
        for (row, line) in s.lines().enumerate() {
            trees.push(vec![]);
            for (_, c) in line.chars().enumerate() {
                trees[row].push(c.to_digit(10).unwrap());
            }
        }
        Ok(Self { trees })
    }
}

common::test!(day08, (21, 8), (1829, 291840));
