use common::graph::Graph;

pub fn input() -> &'static str {
    include_str!("../../input/day15.txt")
}

pub fn test_input() -> &'static str {
    include_str!("../../input/tests/day15.txt")
}

pub fn solve(input: &str) -> (usize, usize) {
    let width = input.lines().next().unwrap().len();
    let weights = weights(input);

    let g = Graph::from(&weights);
    let part_one = g.dijkstra(0, g.nodes.len() - 1).unwrap();

    let extended = extend(&weights, width, width * 5);
    let g = Graph::from(&extended);
    let part_two = g.dijkstra(0, g.nodes.len() - 1).unwrap();

    (part_one, part_two)
}

fn weights(input: &str) -> Vec<Vec<usize>> {
    let mut weights: Vec<Vec<usize>> = vec![];
    for (r, row) in input.lines().enumerate() {
        weights.push(vec![]);
        for c in row.chars() {
            weights[r].push(c.to_digit(10).unwrap() as usize);
        }
    }
    weights
}

fn extend(initial: &Vec<Vec<usize>>, initial_dimensions: usize, height: usize) -> Vec<Vec<usize>> {
    let mut weights: Vec<Vec<usize>> = vec![vec![0; initial.len() * 5]; initial.len() * 5];
    for c in 0..initial_dimensions {
        let extended = extend_col(
            &initial
                .iter()
                .enumerate()
                .map(|(row, _)| initial[row][c])
                .collect(),
            initial_dimensions,
            5,
        );
        for r in 0..height {
            weights[r][c] = extended[r]
        }
    }
    for r in 0..height {
        weights[r].truncate(initial_dimensions);
        weights[r] = extend_row(&weights[r], initial_dimensions, 5);
    }
    weights
}

fn extend_row(initial: &Vec<usize>, width: usize, factor: usize) -> Vec<usize> {
    let mut weights: Vec<usize> = initial.clone();
    let len = weights.len();
    for i in 0..factor - 1 {
        let offset = i * len;
        let w = weights.clone();
        let mapped = w[offset..offset + width]
            .into_iter()
            .map(|value| match value + 1 {
                v @ 1..=9 => v,
                _ => 1,
            });
        weights.extend(mapped);
    }
    weights
}

fn extend_col(initial: &Vec<usize>, height: usize, factor: usize) -> Vec<usize> {
    let mut weights: Vec<usize> = initial.clone();
    let len = weights.len();
    for i in 0..factor - 1 {
        let offset = i * len;
        let w = weights.clone();
        let mapped = w[offset..offset + height]
            .into_iter()
            .map(|value| match value + 1 {
                v @ 1..=9 => v,
                _ => 1,
            });
        weights.extend(mapped);
    }
    weights
}

common::test!(day15, (40, 315), (361, 2838));
