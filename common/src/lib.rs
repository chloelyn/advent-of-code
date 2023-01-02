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
