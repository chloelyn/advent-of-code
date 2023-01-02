const CODE: &str = include_str!("solve.py");

pub fn input() -> &'static str {
    include_str!("../../../input/day07.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../../input/tests/day07.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    common::python::solve(input, CODE)
}

common::test!(day07, (95437, 24933642), (1555642, 5974547));
