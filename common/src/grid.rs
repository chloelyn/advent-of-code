pub fn neighbors(
    (width, height): (usize, usize),
    (row, col): (usize, usize),
    diagonals: bool,
) -> impl IntoIterator<Item = (usize, usize)> {
    let mut checks: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    if diagonals {
        checks.extend([(1, 1), (1, -1), (-1, 1), (-1, -1)].iter())
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

pub fn manhattan_distance((x1, y1): (isize, isize), (x2, y2): (isize, isize)) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub fn exists((width, height): (usize, usize), (row, col): (usize, usize)) -> bool {
    row < height && col < width
}
