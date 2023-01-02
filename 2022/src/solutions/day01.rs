use common::Solution;

pub struct DayOne;

const INPUT: &str = include_str!("../../input/day01.txt");

impl Solution for DayOne {
    fn solve() {
        let input: Vec<_> = INPUT.trim().split("\n\n").collect::<Vec<_>>();
        let mut calorie_counts: Vec<usize> = vec![];
        for elf in &input {
            let mut total_calories: usize = 0;
            for calories in &elf.trim().split("\n").collect::<Vec<_>>() {
                total_calories += calories.parse::<usize>().unwrap();
            }
            calorie_counts.push(total_calories);
        }
        calorie_counts.sort();
        println!("Part One: {}", calorie_counts.last().unwrap());
        println!(
            "Part Two: {}",
            calorie_counts.iter().rev().take(3).sum::<usize>()
        );
    }
}
