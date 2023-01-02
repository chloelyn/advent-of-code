use std::sync::RwLock;

type Stacks = Vec<RwLock<Vec<char>>>;

pub fn input() -> (&'static str, (usize, usize)) {
    (include_str!("../../input/day05.txt"), (10, 8))
}

pub fn test_input() -> (&'static str, (usize, usize)) {
    (include_str!("../../input/tests/day05.txt"), (5, 3))
}

pub fn solve((input, offsets): (&str, (usize, usize))) -> (String, String) {
    (
        move_crates(input, offsets, true),
        move_crates(input, offsets, false),
    )
}

fn move_crates(input: &str, offsets: (usize, usize), part_one: bool) -> String {
    let stacks = parse_stacks(input, offsets.1);
    input
        .lines()
        .skip(offsets.0)
        .map(|step| {
            let s: Vec<&str> = step.split(' ').collect();
            let parse = |s: &str| s.parse::<usize>().unwrap();
            (parse(s[1]), parse(s[3]), parse(s[5]))
        })
        .for_each(|(count, from, to)| {
            let from = &mut stacks[from - 1].write().unwrap();
            let to = &mut stacks[to - 1].write().unwrap();
            let len = from.len();
            let moving = &from[len - count..];
            if part_one {
                to.extend(moving.iter().rev());
            } else {
                to.extend(moving.iter());
            }
            from.truncate(len - count);
        });
    stacks
        .iter()
        .map(|stack| stack.read().unwrap())
        .map(|stack| format!("{}", stack.last().unwrap()))
        .reduce(|acc, s| acc + s.as_str())
        .unwrap()
}

fn parse_stacks(input: &str, offset: usize) -> Stacks {
    let mut stacks: Stacks = Vec::new();
    let mut cur = 0;
    input
        .lines()
        .take(offset)
        .enumerate()
        .for_each(|(idx, line)| {
            line.chars().skip(1).step_by(4).for_each(|c| {
                if idx == 0 {
                    stacks.push(RwLock::new(vec![]));
                }
                if c != ' ' {
                    stacks[cur].write().unwrap().push(c);
                }
                cur += 1;
            });
            cur = 0;
        });
    stacks
        .iter()
        .for_each(|stack| stack.write().unwrap().reverse());

    stacks
}

common::test!(
    day05,
    (String::from("CMZ"), String::from("MCD")),
    (String::from("ZBDRNPMVH"), String::from("WDLPFNNNB"))
);
