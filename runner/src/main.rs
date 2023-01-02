use common::Solution;
use std::ops::Sub;
use twenty_two::solutions::*;

const MAX_ITERATIONS: u128 = 1000;

macro_rules! table {
    {$( ($x:expr, $name:expr, time = $time:expr) ),*} => {
        println!(
            "{0: <3} | {1: <10} | {2: <10} | {3: <5}",
            "day", "part one", "part two", "avg time"
        );
        $(
            time($x, $name, $time);
        )*
    };
}

fn time(solution: impl Solution, name: &str, time: bool) {
    let mut average: Option<u128> = None;
    if time {
        let sum: u128 = (0..MAX_ITERATIONS)
            .into_iter()
            .map(|_| {
                let start = std::time::Instant::now();
                solution.solve();
                let end = std::time::Instant::now();
                let elapsed = end.sub(start);

                elapsed.as_micros()
            })
            .sum();

        average = Some(sum / MAX_ITERATIONS);
    }

    let (part_one, part_two) = solution.solve();
    let average = match average {
        Some(average) => format!("{}μs", average),
        _ => String::from("n/a"),
    };
    println!(
        "{0: <3} | {1: <10} | {2: <10} | {3: <5}",
        name, part_one, part_two, average
    );
}

fn main() {
    table! {
        (day01::DayOne, "01", time = true)
    };
}
