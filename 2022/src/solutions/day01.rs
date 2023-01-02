pub fn input() -> &'static str {
    include_str!("../../input/day01.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day01.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let input: Vec<_> = input.split("\n\n").collect::<Vec<_>>();
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

common::test!(day01, (24000, 45000), (69836, 207968));
