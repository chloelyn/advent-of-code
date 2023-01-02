pub use criterion::{black_box, criterion_group, criterion_main, Criterion};
pub use regex;

pub mod graph;
pub mod grid;
pub mod macros;
pub mod ocr;
pub mod python;

pub mod par {
    pub use rayon::prelude::*;
}
