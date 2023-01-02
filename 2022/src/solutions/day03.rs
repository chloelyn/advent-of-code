pub fn input() -> &'static str {
    include_str!("../../input/day03.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day03.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let part_one = input
        .lines()
        .into_iter()
        .map(|row| {
            let (first, second) = row.split_at(row.len() / 2);
            let c = get_badge(first, |&c| second.contains([c]));
            value_of(c)
        })
        .sum::<u64>();

    let part_two = input
        .lines()
        .array_chunks::<3>()
        .map(|group| {
            let [first, second, third] = group;
            let c = get_badge(first, |&c| second.contains([c]) && third.contains([c]));
            value_of(c)
        })
        .sum::<u64>();

    (part_one as usize, part_two as usize)
}

fn get_badge<F>(first: &str, f: F) -> char
where
    F: FnMut(&char) -> bool,
{
    first
        .chars()
        .filter(f)
        .take(1)
        .next()
        .unwrap_or_else(|| panic!("invalid input"))
}

fn value_of(c: char) -> u64 {
    if c.is_ascii_lowercase() {
        (26 - (122 - c as u8)) as u64
    } else {
        (52 - (90 - c as u8)) as u64
    }
}

common::test!(day03, (157, 70), (8053, 2425));
