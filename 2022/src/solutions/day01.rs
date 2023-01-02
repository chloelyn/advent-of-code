use common::Solution;

pub struct Day01;

impl Solution for Day01 {
    fn input<'a>() -> &'a str {
        include_str!("../../input/day01.txt")
    }

    fn solve(&self) -> (usize, usize) {
        let input: Vec<_> = Day01::input().split("\n\n").collect::<Vec<_>>();
        let mut calorie_counts: Vec<usize> = vec![];
        for elf in &input {
            let mut total_calories: usize = 0;
            for calories in &elf.split("\n").collect::<Vec<_>>() {
                total_calories += calories.parse::<usize>().unwrap();
            }
            calorie_counts.push(total_calories);
        }
        calorie_counts.sort();
        (
            *calorie_counts.last().unwrap(),
            calorie_counts.iter().rev().take(3).sum::<usize>(),
        )
    }
}
