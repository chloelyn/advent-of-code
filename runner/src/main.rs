use catppuccin::MOCHA;
use common::Solution;
use std::ops::Sub;
use tabled::{format::Format, object::Columns, Modify, Style, Tabled};
use twenty_two::solutions::*;

const MAX_ITERATIONS: u128 = 1000;

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct SolveData {
    day: String,
    part_one: usize,
    part_two: usize,
    avg_time: String,
}

macro_rules! table {
    {$( ($x:expr, $name:expr, time = $time:expr) ),*} => {{
        let mut data: Vec<SolveData> = vec![];
        $(
            data.push(time($x, $name, $time));
        )*
        let mut table = tabled::Table::new(data);
        table
            .with(Style::rounded())
            .with(Modify::new(Columns::single(0)).with(Format::new(|s| MOCHA.red.ansi_paint(s).to_string())))
            .with(Modify::new(Columns::new(1..=2)).with(Format::new(|s| MOCHA.green.ansi_paint(s).to_string())))
            .with(Modify::new(Columns::new(3..)).with(Format::new(|s| MOCHA.yellow.ansi_paint(s).to_string())));
        table
    }};
}

fn time(solution: impl Solution, day: &str, time: bool) -> SolveData {
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

    SolveData {
        day: day.into(),
        part_one,
        part_two,
        avg_time: match average {
            Some(average) => format!("{}μs", average),
            _ => String::from("n/a"),
        },
    }
}

fn main() {
    println!(
        "{}",
        table! {
            (day01::DayOne, "01", time = false),
            (day02::DayTwo, "02", time = false)
        }
    )
}
