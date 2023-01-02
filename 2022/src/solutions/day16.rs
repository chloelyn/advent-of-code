use common::regex::Regex;
use std::{cell::LazyCell, collections::HashMap, error::Error, str::FromStr};

const RE: LazyCell<Regex> = LazyCell::new(|| Regex::new("[A-Z]{2}|[0-9]{1,2}").unwrap());

pub fn input() -> &'static str {
    include_str!("../../input/day16.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day16.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    (0, 0)
}

struct TunnelSystem {
    valves: HashMap<String, Valve>,
}

impl TunnelSystem {
    fn new(input: &str) -> Self {
        let mut valves: HashMap<String, Valve> = HashMap::new();
        for line in input.lines() {
            let valve = Valve::from_str(line).unwrap();
            valves.insert(valve.name.clone(), valve);
        }
        Self { valves }
    }

    fn get(&self, valve: &str) -> &Valve {
        self.valves.get(&String::from(valve)).unwrap()
    }
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
    tunnels: Vec<String>,
}

impl FromStr for Valve {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RE.captures_iter(s).collect::<Vec<_>>();
        Ok(Self {
            name: captures[0][0].to_string(),
            flow_rate: captures[1][0].parse::<usize>().unwrap(),
            tunnels: captures[2..]
                .iter()
                .map(|capture| capture[0].to_string())
                .collect::<Vec<_>>(),
        })
    }
}

common::test!(day16, (0, 0), (0, 0));
