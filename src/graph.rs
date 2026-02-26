use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

/*
Gaurav Sablok
codeprog@icloud.com

Graph annotation of the Gff3 so that graph machine learning can be called on the node and edges.
*/

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error: {}", self.message)
    }
}

impl Error for ParseError {}

impl ParseError {
    fn new(msg: &str) -> Self {
        ParseError {
            message: msg.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub sequence: String,
    pub species: String,
    pub start_pos: i32,
    pub strand_rank: i32,
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<String, Node>,
    pub edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: String, sequence: String, species: String, start: i32, sr: i32) {
        self.nodes.insert(
            id.clone(),
            Node {
                id,
                sequence,
                species,
                start_pos: start,
                strand_rank: sr,
            },
        );
    }

    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges
            .entry(from)
            .or_insert_with(HashSet::new)
            .insert(to);
    }

    pub fn print_graph(&self) {
        println!("=== NODES ===");
        for (id, node) in &self.nodes {
            let orient = if node.strand_rank == 1 { "-" } else { "+" };
            println!(
                "{} ({}) [{}] pos:{} len:{}",
                id,
                orient,
                node.species,
                node.start_pos,
                node.sequence.len()
            );
        }

        println!("\n=== EDGES ===");
        for (from, targets) in &self.edges {
            for to in targets {
                println!("{} -> {}", from, to);
            }
        }
    }

    pub fn dfs(&self, start: &str) -> Vec<Vec<String>> {
        let mut results = Vec::new();
        let mut visited = HashMap::new();
        let mut path = Vec::new();

        self.dfs_recursive(start, &mut visited, &mut path, &mut results);
        results
    }

    fn dfs_recursive<'a>(
        &'a self,
        current: &'a str,
        visited: &mut HashMap<&'a str, bool>,
        path: &mut Vec<String>,
        results: &mut Vec<Vec<String>>,
    ) {
        visited.insert(current, true);
        path.push(current.to_string());

        let neighbors = self.edges.get(current).unwrap_or(&HashSet::new().clone());

        if neighbors.is_empty() {
            results.push(path.clone());
        } else {
            for neighbor in neighbors {
                if !visited.get(neighbor.as_str()).unwrap_or(&false) {
                    self.dfs_recursive(neighbor, visited, path, results);
                }
            }
        }
        path.pop();
        visited.insert(current, false);
    }
}

// Unit tests
