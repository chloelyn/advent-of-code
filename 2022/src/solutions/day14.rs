use std::{
    cmp::{max, min},
    collections::HashMap,
};

pub fn input() -> &'static str {
    include_str!("../../input/day14.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day14.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut infinite_cave = Cave::new(input);
    let mut finite_cave = Cave::new(input);
    (
        (0..).map_while(|_| infinite_cave.spawn()).count(),
        (0..).map_while(|_| finite_cave.spawn_with_floor()).count() + 1,
    )
}

struct Cave {
    cave: HashMap<(usize, usize), char>,
    highest: usize,
}

impl Cave {
    fn new(input: &str) -> Self {
        let mut cave: HashMap<(usize, usize), char> = HashMap::new();
        for line in pairs(input) {
            for window in line.windows(2) {
                let [first, second] = window else { unimplemented!() };
                let (low, high) = if first.0 == second.0 {
                    (min(first.1, second.1), max(first.1, second.1))
                } else {
                    (min(first.0, second.0), max(first.0, second.0))
                };
                if first.0 == second.0 {
                    (low..=high).for_each(|y| {
                        cave.insert((first.0, y), '#');
                    });
                } else {
                    (low..=high).for_each(|x| {
                        cave.insert((x, first.1), '#');
                    });
                }
            }
        }
        Self {
            highest: cave.keys().max_by(|&&a, &&b| a.1.cmp(&b.1)).unwrap().1,
            cave,
        }
    }

    fn get(&mut self, coordinate: (usize, usize)) -> char {
        let air = &mut '.';
        let mut next = self.cave.get_mut(&coordinate);
        next.get_or_insert(air).to_owned().to_owned()
    }

    fn step(&mut self, sand: (usize, usize)) -> (isize, isize, bool) {
        let down = self.get((sand.0, sand.1 + 1));
        let left = self.get((sand.0 - 1, sand.1 + 1));
        let right = self.get((sand.0 + 1, sand.1 + 1));
        match (down, left, right) {
            ('.', _, _) => (0, 1, false),
            ('#' | 'o', '.', _) => (-1, 1, false),
            ('#' | 'o', '#' | 'o', '.') => (1, 1, false),
            ('#' | 'o', '#' | 'o', '#' | 'o') => (0, 0, true),
            _ => unreachable!(),
        }
    }

    fn spawn(&mut self) -> Option<(usize, usize)> {
        let mut sand = (500, 0);
        loop {
            if sand.1 > self.highest {
                return None;
            }
            let (dx, dy, b) = self.step(sand);
            if b {
                break;
            }
            let x = sand.0 as isize;
            let y = sand.1 as isize;
            sand.0 = (x + dx) as usize;
            sand.1 = (y + dy) as usize;
        }
        self.cave.insert(sand, 'o');
        Some(sand)
    }

    fn spawn_with_floor(&mut self) -> Option<(usize, usize)> {
        let mut sand = (500, 0);
        loop {
            if sand.1 == self.floor() - 1 {
                break;
            }
            let (dx, dy, b) = self.step(sand);
            if b {
                break;
            }
            let x = sand.0 as isize;
            let y = sand.1 as isize;
            sand.0 = (x + dx) as usize;
            sand.1 = (y + dy) as usize;
        }
        if sand == (500, 0) {
            return None;
        }
        self.cave.insert(sand, 'o');
        Some(sand)
    }

    fn floor(&self) -> usize {
        self.highest + 2
    }
}

fn pairs(input: &str) -> impl IntoIterator<Item = Vec<(usize, usize)>> + '_ {
    input.lines().map(|line| {
        line.split("->")
            .map(|coordinate| coordinate.trim().split_once(',').unwrap())
            .map(|(first, second)| {
                let first: usize = first.parse().unwrap();
                let second: usize = second.parse().unwrap();
                (first, second)
            })
            .collect::<Vec<_>>()
    })
}

common::test!(day14, (24, 93), (672, 26831));
