use common::{
    grid,
    par::{IntoParallelIterator, ParallelIterator},
    regex::Regex,
};
use std::{
    cell::LazyCell,
    cmp::{max, min},
    error::Error,
    ops::Deref,
    str::FromStr,
};

const RE: LazyCell<Regex> = LazyCell::new(|| Regex::new("(x|y)=(-?[0-9]+)").unwrap());

pub fn input() -> (&'static str, isize, isize) {
    (include_str!("../../input/day15.txt"), 2_000_000, 4_000_000)
}

pub fn test_input() -> (&'static str, isize, isize) {
    (include_str!("../../input/tests/day15.txt"), 10, 20)
}

pub fn solve((input, y, _): (&str, isize, isize)) -> (usize, usize) {
    let reports = Reports::from_str(input).unwrap();
    (reports.no_beacon(y), 0)
}

struct Reports(Vec<Report>);

impl Reports {
    fn min(&self) -> isize {
        self.iter()
            .map(|a| min(a.sensor.0, a.beacon.0))
            .min()
            .unwrap()
    }

    fn max(&self) -> isize {
        self.iter()
            .map(|a| max(a.sensor.0, a.beacon.0))
            .max()
            .unwrap()
    }

    fn no_beacon(&self, y: isize) -> usize {
        // TODO: figure out the proper bounds for this
        (self.min() - 100_000_000..=self.max() + 100_000_000)
            .into_par_iter()
            .filter_map(|x| {
                self.iter().find(|report| {
                    grid::manhattan_distance((x, y), report.sensor)
                        <= grid::manhattan_distance(report.beacon, report.sensor)
                        && !(self.sensors().any(|sensor| sensor.0 == x && sensor.1 == y)
                            || self.beacons().any(|sensor| sensor.0 == x && sensor.1 == y))
                })
            })
            .count()
    }

    fn sensors(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.iter().map(|report| report.sensor)
    }

    fn beacons(&self) -> impl Iterator<Item = (isize, isize)> + '_ {
        self.iter().map(|report| report.beacon)
    }
}

impl Deref for Reports {
    type Target = Vec<Report>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for Reports {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .map(|sensor| Report::from_str(sensor).unwrap())
                .collect::<Vec<_>>(),
        ))
    }
}

#[derive(Debug)]
struct Report {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

impl FromStr for Report {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut location, mut beacon) = ((0, 0), (0, 0));
        for (i, [x, y]) in RE.captures_iter(s).array_chunks::<2>().enumerate() {
            let &x = &x[2].parse::<isize>().unwrap();
            let &y = &y[2].parse::<isize>().unwrap();
            match i {
                0 => location = (x, y),
                1 => beacon = (x, y),
                _ => unreachable!(),
            }
        }
        Ok(Self {
            sensor: location,
            beacon,
        })
    }
}

common::test!(day15, (26, 0), (4665948, 0));
