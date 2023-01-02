#[derive(Debug)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub const fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![],
            width,
            height,
        }
    }

    pub fn get(&self, (row, col): (usize, usize)) -> &T {
        if !self.exists((row, col)) {
            panic!(
                "Invalid (row, col) pair: ({row}, {col}) for size: ({}, {})",
                self.width, self.height
            )
        } else {
            &self.data[row][col]
        }
    }

    pub fn set(&mut self, (row, col): (usize, usize), value: T) {
        if !self.exists((row, col)) {
            panic!(
                "Invalid (row, col) pair: ({row}, {col}) for size: ({}, {})",
                self.width, self.height
            )
        } else {
            self.data[row][col] = value;
        }
    }

    const fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn neighbors(&self, (row, col): (usize, usize), diagonals: bool) -> Vec<(usize, usize)> {
        let mut checks: Vec<(isize, isize)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        if diagonals {
            checks.extend([(1, 1), (1, -1), (-1, 1), (-1, -1)].iter())
        }

        if !self.exists((row, col)) {
            panic!(
                "Invalid (row, col) pair: ({row}, {col}) for size: ({}, {})",
                self.width, self.height
            );
        }

        let mut neighbors: Vec<(usize, usize)> = vec![];
        for (radjust, cadjust) in checks {
            let r = (row as isize + radjust).try_into();
            let c = (col as isize + cadjust).try_into();

            if let (Ok(r), Ok(c)) = (r, c) {
                neighbors.push((r, c));
            } else {
                continue;
            };
        }

        neighbors
    }

    pub fn exists(&self, (row, col): (usize, usize)) -> bool {
        if self.data.is_empty() {
            return false;
        }
        let (width, height) = self.dimensions();
        row < height && col < width
    }
}
