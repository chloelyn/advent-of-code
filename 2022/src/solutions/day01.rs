use common::Solution;

pub struct DayOne;

impl Solution for DayOne {
    fn input<'a>() -> &'a str {
        include_str!("../../input/day01.txt")
    }

    fn solve(&self) -> (usize, usize) {
        let input: Vec<_> = DayOne::input().split("\n\n").collect::<Vec<_>>();
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
