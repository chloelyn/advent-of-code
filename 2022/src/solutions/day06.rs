use std::collections::HashSet;

const SOP_LEN: usize = 4;
const SOM_LEN: usize = 14;

pub fn input() -> &'static str {
    include_str!("../../input/day06.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day06.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let mut chars: Vec<char> = Vec::with_capacity(input.len());
    input.chars().collect_into(&mut chars);

    (find_index(&chars, SOP_LEN), find_index(&chars, SOM_LEN))
}

fn find_index(chars: &Vec<char>, size: usize) -> usize {
    chars
        .windows(size)
        .enumerate()
        .find_map(|(idx, window)| {
            let mut s: HashSet<char> = HashSet::new();
            s.extend(window.iter());
            if s.len() == size {
                Some(idx + size)
            } else {
                None
            }
        })
        .unwrap()
}

common::test!(day06, (7, 19), (1582, 3588));
