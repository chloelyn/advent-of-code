#[macro_export]
macro_rules! test {
    ($module:ident, $test_output:expr, $output:expr) => {
        #[cfg(test)]
        mod tests {
            use crate::solutions::$module::{input, solve, test_input};

            #[test]
            fn example() {
                assert_eq!(solve(test_input()), $test_output);
            }

            #[test]
            fn solution() {
                assert_eq!(solve(input()), $output);
            }
        }
    };
}

#[macro_export]
macro_rules! benchmark {
    ($year:ident, {$($module:ident),*}) => {
        use $year::solutions::*;

        fn criterion_benchmark(c: &mut $crate::Criterion) {
            $(
                c.bench_function(stringify!($module), |b| {
                    b.iter(|| $module::solve($crate::black_box($module::input())))
                });
            )*
        }

        $crate::criterion_group!(benches, criterion_benchmark);
        $crate::criterion_main!(benches);
    };
}

#[macro_export]
macro_rules! indented {
    { $string:literal } => {{
        $string
            .lines()
            .map(|line| line.trim_start())
            .fold(String::new(), |acc, s| acc + s + "\n")
            .trim()
            .to_string()
    }}
}
