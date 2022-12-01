use std::io::{self, BufRead};

#[derive(Clone)]
struct Node {
    name: String,
    large: bool,
    links: Vec<usize>,
}

#[derive(Clone)]
struct Graph {
    vertices: Vec<Node>,
    edges: Vec<(usize, usize)>
}

impl Graph {
    fn new() -> Graph {
        Graph {
            vertices: vec![],
            edges: vec![],
        }
    }

    fn add_edge(&mut self, from: &str, to: &str) {
        self.add_directional_edge(from, to);
        self.add_directional_edge(to, from);
    }

    fn add_directional_edge(&mut self, from: &str, to: &str) {
        let from_idx = self.node_index(from);
        let to_idx = self.node_index(to);
        self.edges.push((from_idx, to_idx));
        self.vertices[from_idx].links.push(to_idx);
    }

    fn node_index(&mut self, name: &str) -> usize {
        // find or add edge
        let len = self.vertices.len();
        for i in 0..len {
            if self.vertices[i].name == name {
                return i;
            }
        }
        return self.add_node(name);
    }

    fn node(&self, name: &str) -> Option<&Node> {
        let len = self.vertices.len();
        for i in 0..len {
            if self.vertices[i].name == name {
                return Some(&self.vertices[i]);
            }
        }
        None
    }

    fn add_node(&mut self, name: &str) -> usize {
        let node = Node {
            name: String::from(name),
            large: name.chars().all(char::is_uppercase),
            links: vec![]
        };
        self.vertices.push(node);
        self.vertices.len() - 1
    }

    fn without_node(&self, name: &str) -> Graph {
        let mut graph = Graph {
            vertices: vec![],
            edges: vec![],
        };
        for (from_idx, to_idx) in self.edges.iter() {
            let from = &self.vertices[*from_idx].name;
            let to = &self.vertices[*to_idx].name;
            if from != name && to != name {
                graph.add_directional_edge(&from, &to);
            }
        }
        graph
    }

    fn paths(&self, from: &str, to: &str) -> Vec<Vec<String>> {
        let maybe_from_node = self.node(from);
        let mut paths: Vec<Vec<String>> = vec![];
        if maybe_from_node.is_none() {
            return paths;
        }
        let from_node = maybe_from_node.unwrap();
        let linked_nodes = from_node.links.iter().map(|i| &self.vertices[*i]).collect::<Vec<_>>();

        for node in linked_nodes {
            if node.name == to {
                let path = vec![String::from(&node.name)];
                paths.push(path);
            } else {
                let subgraph = if from_node.large {
                    self.clone()
                } else {
                    self.without_node(from)
                };
                let subpaths = subgraph.paths(&node.name, to);
                for subpath in subpaths {
                    let mut path = vec![String::from(&node.name)];
                    for subpath_node in subpath {
                        path.push(subpath_node);
                    }
                    paths.push(path);
                }
            }
        }

        paths
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for (from_idx, to_idx) in self.edges.iter() {
            let from = &self.vertices[*from_idx];
            let to = &self.vertices[*to_idx];
            write!(f, "{}-{},", from.name, to.name)?
        }
        Ok(())
    }
}

fn read_input() -> Graph {
    let mut graph = Graph::new();
    io::stdin().lock().lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .for_each(|line| {
            let words = line.split(|c: char| !c.is_alphabetic()).collect::<Vec<_>>();
            graph.add_edge(words[0], words[1]);
        });
    graph
}

fn main() {
    let graph = read_input();

    let paths = graph.paths("start", "end");
    println!("{} paths", paths.len());
}
