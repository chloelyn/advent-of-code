const DECRYPTION_KEY: isize = 811589153;

pub fn input() -> &'static str {
    include_str!("../../input/day20.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day20.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut test: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Number::new(index, line.parse::<isize>().unwrap()))
        .collect();

    let mut actual: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(index, line)| Number::new(index, line.parse::<isize>().unwrap() * DECRYPTION_KEY))
        .collect();

    (
        coordinates(mix(1, &mut test)) as usize,
        coordinates(mix(10, &mut actual)) as usize,
    )
}

fn mix(rounds: usize, file: &mut Vec<Number>) -> Vec<Number> {
    let it = file.clone();
    for _ in 0..rounds {
        for &n in it.iter() {
            let index = file.iter().position(|a| a.index == n.index).unwrap();
            file.remove(index);
            file.insert(
                (index as isize + n.value).rem_euclid(file.len() as isize) as usize,
                n,
            );
        }
    }
    file.clone()
}

fn coordinates(file: Vec<Number>) -> isize {
    let zero = file.iter().position(|n| n.value == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| &file[(zero + offset) % file.len()].value)
        .sum()
}

#[derive(Debug, Copy, Clone)]
struct Number {
    value: isize,
    index: usize,
}

impl Number {
    fn new(index: usize, value: isize) -> Self {
        Self { value, index }
    }
}

common::test!(day20, (3, 1623178306), (7584, 4907679608191));
