pub mod matrix;

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
        use criterion::{black_box, criterion_group, criterion_main, Criterion};
        use $year::solutions::*;

        fn criterion_benchmark(c: &mut Criterion) {
            $(
                c.bench_function(stringify!($module), |b| {
                    b.iter(|| $module::solve(black_box($module::input())))
                });
            )*
        }

        criterion_group!(benches, criterion_benchmark);
        criterion_main!(benches);
    };
}
