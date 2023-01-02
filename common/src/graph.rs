use std::collections::BinaryHeap;

pub struct Graph {
    pub nodes: Vec<Vec<Edge>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

#[derive(Debug)]
pub struct Edge {
    node: usize,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Vec<Vec<usize>>> for Graph {
    /// Generates an adjacency list for a graph from a 2-dimensional array.
    fn from(weights: &Vec<Vec<usize>>) -> Self {
        let width = weights.iter().next().unwrap().len();
        let height = weights.len();
        let mut nodes: Vec<Vec<Edge>> = vec![];
        for (row, values) in weights.iter().enumerate() {
            for (col, _) in values.iter().enumerate() {
                let mut edges: Vec<Edge> = vec![];
                for edge in super::grid::neighbors((width, height), (row, col), false) {
                    edges.push(Edge {
                        node: width * edge.0 + edge.1,
                        cost: weights[edge.0][edge.1],
                    })
                }
                nodes.push(edges);
            }
        }
        Self { nodes }
    }
}

impl Graph {
    pub fn dijkstra(&self, start: usize, goal: usize) -> Option<usize> {
        let mut dist: Vec<_> = (0..self.nodes.len()).map(|_| usize::MAX).collect();
        let mut heap = BinaryHeap::new();

        dist[start] = 0;
        heap.push(State {
            cost: 0,
            position: start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == goal {
                return Some(cost);
            }

            if cost > dist[position] {
                continue;
            }

            for edge in &self.nodes[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
                };

                if next.cost < dist[next.position] {
                    heap.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }

        None
    }
}
