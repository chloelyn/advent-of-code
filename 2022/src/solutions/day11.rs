use std::{
    cell::RefCell,
    error::Error,
    iter::Skip,
    str::{FromStr, Lines},
};

pub fn input() -> &'static str {
    include_str!("../../input/day11.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day11.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    (
        simulate(MonkeyGroup::from_str(input).unwrap(), 20, true),
        simulate(MonkeyGroup::from_str(input).unwrap(), 10_000, false),
    )
}

fn simulate(mut group: MonkeyGroup, end: usize, relieving: bool) -> usize {
    for _ in 0..end {
        group.simulate(relieving);
    }
    group.monkey_business()
}

struct MonkeyGroup {
    monkeys: Vec<RefCell<Monkey>>,
}

impl MonkeyGroup {
    fn simulate(&mut self, relieving: bool) {
        let modulus = self.modulus();
        for monkey in self.monkeys.iter() {
            for inspected in monkey.borrow().inspections(relieving, modulus) {
                let mut next = self.monkeys[inspected.1].borrow_mut();
                next.items.push(inspected.0);
            }
            let mut monkey = monkey.borrow_mut();
            monkey.inspected += monkey.items.len();
            monkey.items.truncate(0);
        }
    }

    fn modulus(&self) -> usize {
        self.monkeys
            .iter()
            .map(|monkey| monkey.borrow().divisor)
            .reduce(|acc, divisor| acc * divisor)
            .unwrap()
    }

    fn monkey_business(&mut self) -> usize {
        let mut inspected_items: Vec<_> = self
            .monkeys
            .iter()
            .map(|monkey| monkey.borrow().inspected)
            .collect();
        inspected_items.sort_unstable_by(|a, b| b.cmp(a));
        inspected_items[0] * inspected_items[1]
    }
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    inspected: usize,
    success: usize,
    failure: usize,
}

impl Monkey {
    fn inspections(
        &self,
        relieving: bool,
        modulus: usize,
    ) -> impl IntoIterator<Item = (usize, usize)> + '_ {
        self.items.iter().map(move |item| {
            if relieving {
                self.inspect(&item)
            } else {
                self.inspect_modulus(&item, modulus)
            }
        })
    }

    fn inspect(&self, item: &usize) -> (usize, usize) {
        self.next(self.operation.call(*item) / 3)
    }

    fn inspect_modulus(&self, item: &usize, modulus: usize) -> (usize, usize) {
        self.next(self.operation.call(*item) % modulus)
    }

    fn next(&self, worry_level: usize) -> (usize, usize) {
        match worry_level % self.divisor == 0 {
            true => (worry_level, self.success),
            false => (worry_level, self.failure),
        }
    }
}

impl FromStr for MonkeyGroup {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let monkeys = s
            .split("\n\n")
            .map(|monkey| RefCell::new(Monkey::from_str(monkey.trim()).unwrap()))
            .collect();
        Ok(Self { monkeys })
    }
}

impl FromStr for Monkey {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().skip(1);
        let parser = MonkeyParser::new();
        Ok(Monkey {
            items: parser.items(&mut lines),
            operation: parser.operation(&mut lines),
            divisor: parser.test(&mut lines),
            success: parser.outcome(&mut lines, "true"),
            failure: parser.outcome(&mut lines, "false"),
            inspected: 0,
        })
    }
}

struct MonkeyParser;

impl MonkeyParser {
    fn new() -> Self {
        Self {}
    }

    fn items(&self, lines: &mut Skip<Lines>) -> Vec<usize> {
        self.strip(lines, "  Starting items: ")
            .split(',')
            .map(|item| self.parse(item))
            .collect::<Vec<_>>()
    }

    fn operation(&self, lines: &mut Skip<Lines>) -> Operation {
        let line = self.strip(lines, "  Operation: new = old ");
        let (op, n) = line.split_once(' ').unwrap();
        let v = self.parse(n);
        match (op, n) {
            ("+", "old") => Operation::new(move |old: usize| old + old),
            ("*", "old") => Operation::new(move |old: usize| old * old),
            ("*", _) => Operation::new(move |old: usize| old * v),
            ("+", _) => Operation::new(move |old: usize| old + v),
            _ => unreachable!(),
        }
    }

    fn test(&self, lines: &mut Skip<Lines>) -> usize {
        self.parse(&self.strip(lines, "  Test: divisible by "))
    }

    fn outcome(&self, lines: &mut Skip<Lines>, kind: &str) -> usize {
        self.parse(&self.strip(lines, &format!("    If {}: throw to monkey ", kind)))
    }

    fn parse(&self, s: &str) -> usize {
        s.trim().parse::<usize>().unwrap_or(0)
    }

    fn strip(&self, lines: &mut Skip<Lines>, s: &str) -> String {
        lines.next().unwrap().replace(s, "")
    }
}

struct Operation {
    f: Box<dyn Fn(usize) -> usize>,
}

impl Operation {
    fn new<F>(f: F) -> Self
    where
        F: Fn(usize) -> usize + 'static,
    {
        Self { f: Box::new(f) }
    }
}

impl Operation {
    fn call(&self, old: usize) -> usize {
        self.f.as_ref()(old)
    }
}

common::test!(day11, (10605, 2713310158), (54036, 13237873355));
