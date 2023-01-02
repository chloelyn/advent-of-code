const CODE: &str = include_str!("solve.py");

pub fn input() -> &'static str {
    include_str!("../../../input/day13.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../../input/tests/day13.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    common::python::solve(input, CODE)
}

common::test!(day13, (13, 140), (5350, 19570));
