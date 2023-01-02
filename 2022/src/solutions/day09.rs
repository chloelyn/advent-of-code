use std::collections::HashSet;

const STEP_DISTANCE: isize = 2;

pub fn input() -> &'static str {
    include_str!("../../input/day09.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day09.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut short_rope = Rope::new(1);
    let mut rope = Rope::new(9);
    for line in input.lines() {
        let (direction, count) = line.split_once(' ').unwrap();
        let count = count.parse::<usize>().unwrap();
        short_rope.step(direction, count);
        rope.step(direction, count);
    }
    (short_rope.visited.len(), rope.visited.len())
}

struct Rope {
    head: (isize, isize),
    rest: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            head: (0, 0),
            rest: vec![(0, 0); length],
            visited: HashSet::new(),
        }
    }

    fn offset(&mut self, front: (isize, isize), idx: usize) -> (isize, isize) {
        let behind = self.rest[idx];
        match (front, behind) {
            _ if front.0 == behind.0 && front.1 > behind.1 => (0, 1),
            _ if front.0 == behind.0 && front.1 < behind.1 => (0, -1),
            _ if front.1 == behind.1 && front.0 < behind.0 => (-1, 0),
            _ if front.1 == behind.1 && front.0 > behind.0 => (1, 0),
            _ if front.0 < behind.0 && front.1 < behind.1 => (-1, -1),
            _ if front.0 < behind.0 && front.1 > behind.1 => (-1, 1),
            _ if front.0 > behind.0 && front.1 < behind.1 => (1, -1),
            _ if front.0 > behind.0 && front.1 > behind.1 => (1, 1),
            (_, _) => unreachable!(),
        }
    }

    fn step_next(&mut self, front: (isize, isize), idx: usize) {
        let mut behind = self.rest[idx];
        if self.should_step(front, behind) {
            let offset = self.offset(front, idx);
            behind.0 += offset.0;
            behind.1 += offset.1;
        }
        self.rest[idx] = behind;
    }

    fn step_direction(&mut self) {
        self.step_next(self.head, 0);
        (1..self.rest.len()).for_each(|i| {
            self.step_next(self.rest[i - 1], i);
        });
        self.visited.insert(self.rest[self.rest.len() - 1]);
    }

    fn step(&mut self, direction: &str, count: usize) {
        match direction {
            "L" => (0..count).for_each(|_| {
                self.head.1 -= 1;
                self.step_direction();
            }),
            "R" => (0..count).for_each(|_| {
                self.head.1 += 1;
                self.step_direction();
            }),
            "U" => (0..count).for_each(|_| {
                self.head.0 -= 1;
                self.step_direction();
            }),
            "D" => (0..count).for_each(|_| {
                self.head.0 += 1;
                self.step_direction();
            }),
            _ => unreachable!(),
        }
    }

    fn should_step(&self, front: (isize, isize), behind: (isize, isize)) -> bool {
        let (head_row, head_col) = front;
        let (tail_row, tail_col) = behind;
        let (row_diff, col_diff) = ((head_row - tail_row).abs(), (head_col - tail_col).abs());
        let horizontal = row_diff == STEP_DISTANCE && head_col == tail_col;
        let vertical = col_diff == STEP_DISTANCE && head_row == tail_row;
        let diagonal = (head_row != tail_row && head_col != tail_col)
            && (row_diff == STEP_DISTANCE || col_diff == STEP_DISTANCE);
        (horizontal || vertical) || diagonal
    }
}

common::test!(day09, (13, 1), (6087, 2493));
