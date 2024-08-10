pub struct Graph {
    pub adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(vertices: usize) -> Self {
        Self {
            adj_list: vec![Vec::new(); vertices],
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list[u].push(v);
        self.adj_list[v].push(u); // Assuming undirected graph
    }
}

// TODO: Implement other graph operations (e.g., remove edge, search).
// TODO: Write unit tests for the graph data structure.