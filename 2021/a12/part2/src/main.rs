use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
struct Node {
    name: String,
    large: bool,
}

struct Graph {
    vertices: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

struct Visitor<'g> {
    visited: HashMap<&'g Node, u8>
}

impl Node {
    fn visits(&self, visitor: &Visitor) -> u8 {
        if visitor.visited.contains_key(self) {
            *visitor.visited.get(self).unwrap()
        } else {
            0
        }
    }

    fn visit<'g>(&'g self, visitor: &mut Visitor<'g>) {
        if visitor.visited.contains_key(self) {
            *visitor.visited.get_mut(self).unwrap() += 1;
        } else {
            visitor.visited.insert(self, 1);
        }
    }

    fn unvisit<'g>(&'g self, visitor: &mut Visitor<'g>) {
        if visitor.visited.contains_key(self) {
            *visitor.visited.get_mut(self).unwrap() -= 1;
        }
    }
}

impl Graph {
    fn new() -> Self {
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
        let from_idx = self.get_node_index_adding(from);
        let to_idx = self.get_node_index_adding(to);
        self.edges.push((from_idx, to_idx));
    }

    fn get_node_index_adding(&mut self, name: &str) -> usize {
        // find or add node
        let len = self.vertices.len();
        for i in 0..len {
            if self.vertices[i].name == name {
                return i;
            }
        }
        return self.add_node(name);
    }

    fn get_node_index(&self, name: &str) -> Option<usize> {
        let len = self.vertices.len();
        for i in 0..len {
            if self.vertices[i].name == name {
                return Some(i);
            }
        }
        None
    }

    fn node(&self, name: &str) -> Option<&Node> {
        self.get_node_index(name).map(|i| &self.vertices[i])
    }

    fn add_node(&mut self, name: &str) -> usize {
        let node = Node {
            name: String::from(name),
            large: name.chars().all(char::is_uppercase),
        };
        self.vertices.push(node);
        self.vertices.len() - 1
    }

    fn can_double_visit<'g>(&self, visitor: &Visitor<'g>) -> bool {
        self.vertices.iter().all(|v| v.large || v.visits(visitor) < 2 )
    }

    fn visitable_nodes<'g>(&self, from: &str, visitor: &Visitor<'g>) -> Vec<&Node> {
        let can_double_visit = self.can_double_visit(visitor);
        let from_idx = self.get_node_index(from).unwrap();
        self.edges.iter()
            .filter(|(idx, _)| *idx == from_idx)
            .map(|(_, to_idx)| &self.vertices[*to_idx])
            .filter(|v| v.name != "start")
            .filter(|v| v.large || v.visits(visitor) == 0 || can_double_visit )
            .collect::<Vec<_>>()
    }

    fn paths<'g>(&'g self, from: &str, to: &str, visitor: &mut Visitor<'g>) -> Vec<Vec<String>> {
        let mut paths: Vec<Vec<String>> = vec![];

        for node in self.visitable_nodes(from, visitor) {
            if node.name == to {
                let path = vec![node.name.clone()];
                paths.push(path);
            } else {
                node.visit(visitor);
                let subpaths = self.paths(&node.name, to, visitor);
                node.unvisit(visitor);
                for subpath in subpaths {
                    let mut path = vec![node.name.clone()];
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
        write!(f, "Graph(nodes=[")?;
        for node in self.vertices.iter() {
            write!(f, "{},", node.name)?;
        }
        /*write!(f, "],edges=[");
        for (from_idx, to_idx) in self.edges.iter() {
            let from = &self.vertices[*from_idx];
            let to = &self.vertices[*to_idx];
            write!(f, "{}-{},", from.name, to.name)?
        }*/
        write!(f, "])")?;
        Ok(())
    }
}

fn read_input() -> Graph {
    let mut graph: Graph = Graph::new();
    io::stdin().lock().lines()
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .for_each(|line| {
            let words = line.split(|c: char| !c.is_alphabetic()).map(str::to_owned).collect::<Vec<_>>();
            graph.add_edge(&words[0], &words[1])
        });
    graph
}

#[allow(dead_code)]
fn print_paths(paths: Vec<Vec<String>>) {
    for path in paths {
        print!("start,");
        for node in path {
            print!("{}", node);
            if node != "end" {
                print!(",");
            }
        }
        print!("\n");
    }
}

fn main() {
    let graph = read_input();

    println!("{}", graph);

    let mut visitor = Visitor{visited: HashMap::new()};
    let paths = graph.paths("start", "end", &mut visitor);
    println!("{} paths", paths.len());
    //print_paths(paths);
}
