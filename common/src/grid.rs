pub fn neighbors(
    (width, height): (usize, usize),
    (row, col): (usize, usize),
    diagonals: bool,
) -> impl IntoIterator<Item = (usize, usize)> {
    let mut checks: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if diagonals {
        checks.extend([(1, 1), (1, -1), (-1, 1), (-1, -1)].iter())
    }

    if !exists((width, height), (row, col)) {
        panic!(
            "Invalid (row, col) pair: ({row}, {col}) for size: ({}, {})",
            width, height
        );
    }

    checks.into_iter().filter_map(move |(radjust, cadjust)| {
        let r = (row as isize + radjust).try_into();
        let c = (col as isize + cadjust).try_into();

        match (r, c) {
            (Ok(r), Ok(c)) if exists((width, height), (r, c)) => Some((r, c)),
            (_, _) => None,
        }
    })
}

pub fn exists((width, height): (usize, usize), (row, col): (usize, usize)) -> bool {
    row < height && col < width
}
