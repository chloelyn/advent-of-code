use catppuccin::MOCHA;
use tabled::{format::Format, object::Columns, Modify, Style, Tabled};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct SolveData {
    day: String,
    part_one: String,
    part_two: String,
}

macro_rules! table {
    ($year:ident, {$($module:ident),*}) => {{
        use $year::solutions::*;

        let mut data: Vec<SolveData> = vec![];
        $(
            let s = stringify!($module);
            let day = s.rsplit_once("day").unwrap().1;
            data.push(solve!($module, day));
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

macro_rules! solve {
    ($module:ident, $name:expr) => {{
        let (part_one, part_two) = $module::solve($module::input());

        SolveData {
            day: $name.into(),
            part_one: part_one.to_string(),
            part_two: part_two.to_string(),
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
            day05,
            day06,
            day07,
            day08,
            day09,
            day10
        })
    )
}
