use crate::Loc;

pub struct Map {
    n_nodes: usize,
    nodes: Vec<Loc>,
    graph: Vec<Vec<(usize, f64)>>,
}

impl Map {

    pub fn new() -> Self {

        let nodes: Vec<Loc> = include!("../data/nodes.rs");
        let edges: Vec<(usize, usize)> = include!("../data/edges.rs");

        let n_nodes = nodes.len() - 1;

        let mut graph = vec![vec![]; n_nodes + 1];

        for (x, y) in edges {
            let dist = dist(nodes[x], nodes[y]);
            graph[x].push((y, dist));
            graph[y].push((x, dist));
        }

        Self {
            n_nodes, 
            nodes,
            graph,
        }
    }

    pub fn nearest(&self, loc: Loc) -> usize {
        let mut node = 0;
        let mut nearest = 1e9;
        for i in 1..=self.n_nodes {
            let d = dist(loc, self.nodes[i]);
            if d <= nearest {
                nearest = d;
                node = i;
            }
        }
        node
    }

    pub fn n_nodes(&self) -> usize {
        self.n_nodes
    }

    pub fn loc(&self, x: usize) -> Loc {
        self.nodes[x]
    }

    pub fn to(&self, x: usize) -> &Vec<(usize, f64)> {
        &self.graph[x]
    }

}

fn dist((x0, y0): Loc, (x1, y1): Loc) -> f64 {
    ((x0.abs_diff(x1).pow(2) + y0.abs_diff(y1).pow(2)) as f64).sqrt()
}

pub mod path;

pub mod cycle;