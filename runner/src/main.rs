use catppuccin::MOCHA;
use std::ops::Sub;
use tabled::{format::Format, object::Columns, Modify, Style, Tabled};

const MAX_ITERATIONS: u128 = 1000;

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct SolveData {
    day: String,
    part_one: String,
    part_two: String,
    avg_time: String,
}

macro_rules! table {
    ($year:ident, {$($module:ident),*}) => {{
        use $year::solutions::*;

        let mut data: Vec<SolveData> = vec![];
        $(
            let s = stringify!($module);
            let day = s.rsplit_once("day").unwrap().1;
            data.push(time!($module, day));
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

macro_rules! time {
    ($module:ident, $name:expr) => {{
        let sum: u128 = (0..MAX_ITERATIONS)
            .into_iter()
            .map(|_| {
                let start = std::time::Instant::now();
                $module::solve($module::input());
                let end = std::time::Instant::now();
                let elapsed = end.sub(start);

                elapsed.as_micros()
            })
            .sum();

        let average = sum / MAX_ITERATIONS;

        let (part_one, part_two) = $module::solve($module::input());

        SolveData {
            day: $name.into(),
            part_one: part_one.to_string(),
            part_two: part_two.to_string(),
            avg_time: format!("{}μs", average),
        }
    }};
}

fn main() {
    println!(
        "{}",
        table!(twenty_two, {
            day01,
            day02,
            day03,
            day04,
            day05
        })
    )
}
