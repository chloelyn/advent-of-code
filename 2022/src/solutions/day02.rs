use common::Solution;

pub struct DayTwo;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u64)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Solution for DayTwo {
    fn input<'a>() -> &'a str {
        include_str!("../../input/day02.txt")
    }

    fn solve(&self) -> (usize, usize) {
        let (part_one, part_two): _ = Self::input()
            .split("\n")
            .into_iter()
            .map(|row| {
                row.split(' ')
                    .into_iter()
                    .map(|item| match item.trim() {
                        "A" | "X" => Shape::Rock,
                        "B" | "Y" => Shape::Paper,
                        "C" | "Z" => Shape::Scissors,
                        _ => panic!("invalid input"),
                    })
                    .collect::<Vec<_>>()
            })
            .map(|row| (part_one(&row), part_two(&row)))
            .reduce(|acc, (part_one, part_two)| (acc.0 + part_one, acc.1 + part_two))
            .unwrap();

        (part_one as usize, part_two as usize)
    }
}

fn part_one(row: &Vec<Shape>) -> u64 {
    let a = &row[0];
    let b = &row[1];
    if *a == *b {
        *b as u64 + 3
    } else {
        match (b, a) {
            (Shape::Rock, Shape::Scissors) => *b as u64 + 6,
            (Shape::Paper, Shape::Rock) => *b as u64 + 6,
            (Shape::Scissors, Shape::Paper) => *b as u64 + 6,
            _ => *b as u64,
        }
    }
}

fn part_two(row: &Vec<Shape>) -> u64 {
    let opponent = &row[0];
    let win = &row[1];
    let choices = vec![
        Shape::Rock as u64,
        Shape::Paper as u64,
        Shape::Scissors as u64,
    ];
    match win {
        Shape::Scissors => choices[*opponent as usize % 3] + 6, // win
        Shape::Paper => 3 + *opponent as u64,                   // draw
        Shape::Rock => choices[(*opponent as usize + 1) % 3],   // loss
    }
}
