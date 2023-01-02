use std::sync::RwLock;

type Stacks = Vec<RwLock<Vec<char>>>;

pub fn input() -> (&'static str, bool) {
    (include_str!("../../input/day05.txt"), false)
}

pub fn test_input() -> (&'static str, bool) {
    (include_str!("../../input/tests/day05.txt"), true)
}

pub fn solve((input, test): (&str, bool)) -> (String, String) {
    (
        move_crates(input, test, true),
        move_crates(input, test, false),
    )
}

fn move_crates(input: &str, test: bool, part_one: bool) -> String {
    let stacks = if test {
        get_test_stacks()
    } else {
        get_stacks()
    };
    input
        .lines()
        .map(|step| {
            let s: Vec<&str> = step.split(' ').collect();
            let parse = |s: &str| s.parse::<usize>().unwrap();
            (parse(s[1]), parse(s[3]), parse(s[5]))
        })
        .for_each(|(count, from, to)| {
            let from = &mut stacks[from - 1].write().unwrap();
            let to = &mut stacks[to - 1].write().unwrap();
            let f_len = from.len();
            let moving = &from[f_len - count..];
            if part_one {
                to.extend(moving.iter().rev());
            } else {
                to.extend(moving.iter());
            }
            from.truncate(f_len - count);
        });
    stacks
        .iter()
        .map(|stack| stack.read().unwrap())
        .map(|stack| format!("{}", stack.last().unwrap()))
        .reduce(|acc, s| acc + s.as_str())
        .unwrap()
}

fn get_stacks() -> Stacks {
    vec![
        RwLock::new(vec!['B', 'P', 'N', 'Q', 'H', 'D', 'R', 'T']), // 1
        RwLock::new(vec!['W', 'G', 'B', 'J', 'T', 'V']),           // 2
        RwLock::new(vec!['N', 'R', 'H', 'D', 'S', 'V', 'M', 'Q']), // 3
        RwLock::new(vec!['P', 'Z', 'N', 'M', 'C']),                // 4
        RwLock::new(vec!['D', 'Z', 'B']),                          // 5
        RwLock::new(vec!['V', 'C', 'W', 'Z']),                     // 6
        RwLock::new(vec!['G', 'Z', 'N', 'C', 'V', 'Q', 'L', 'S']), // 7
        RwLock::new(vec!['L', 'G', 'J', 'M', 'D', 'N', 'V']),      // 8
        RwLock::new(vec!['T', 'P', 'M', 'F', 'Z', 'C', 'G']),      // 9
    ]
}

fn get_test_stacks() -> Stacks {
    vec![
        RwLock::new(vec!['Z', 'N']),      // 1
        RwLock::new(vec!['M', 'C', 'D']), // 2
        RwLock::new(vec!['P']),           // 3
    ]
}

common::test!(
    day05,
    (String::from("CMZ"), String::from("MCD")),
    (String::from("ZBDRNPMVH"), String::from("WDLPFNNNB"))
);
