pub use catppuccin::MOCHA;
pub use tabled::{format::Format, object::Columns, Modify, Style, Tabled};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
pub struct SolveData {
    pub day: String,
    pub part_one: String,
    pub part_two: String,
}

#[macro_export]
macro_rules! table {
    ($year:ident, {$($module:ident),*}) => {{
        use $year::solutions::*;
        use $crate::SolveData;

        let mut data: Vec<SolveData> = vec![];
        $(
            let s = stringify!($module);
            let day = s.rsplit_once("day").unwrap().1;
            data.push($crate::solve!($module, day));
        )*
        let mut table = tabled::Table::new(data);
        table
            .with($crate::Style::rounded())
            .with($crate::Modify::new($crate::Columns::single(0)).with($crate::Format::new(|s| $crate::MOCHA.red.ansi_paint(s).to_string())))
            .with($crate::Modify::new($crate::Columns::new(1..=2)).with($crate::Format::new(|s| $crate::MOCHA.green.ansi_paint(s).to_string())))
            .with($crate::Modify::new($crate::Columns::new(3..)).with($crate::Format::new(|s| $crate::MOCHA.yellow.ansi_paint(s).to_string())));
        table
    }};
}

#[macro_export]
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
