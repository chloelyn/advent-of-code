pub mod matrix;

pub trait Solution {
    fn input<'a>() -> &'a str;
    fn solve(&self) -> (usize, usize);
}
