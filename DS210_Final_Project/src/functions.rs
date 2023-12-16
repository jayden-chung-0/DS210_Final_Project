use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap, HashSet, VecDeque}; 

pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<usize>>, 
}

pub fn read_graph(path: &str) -> io::Result<Graph> {
    let file = File::open(path)?;
    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();

    let buf_reader = BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line?;
        if !line_str.starts_with('#') {
            let v: Vec<usize> = line_str
                .to_owned()
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            if v.len() >= 2 {
                let from_node = v[0];
                let to_node = v[1];

                adjacency_list
                    .entry(from_node)
                    .or_insert_with(Vec::new)
                    .push(to_node);
            }
        }
    }

    Ok(Graph { adjacency_list })
}

pub fn distance(graph: &Graph, start_vertex: usize, end_vertex: usize) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(start_vertex);
    visited.insert(start_vertex);

    let mut distance = 0;

    while let Some(current_vertex) = queue.pop_front() {
        if current_vertex == end_vertex {
            return Some(distance);
        }

        if let Some(neighbors) = graph.adjacency_list.get(&current_vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }
        distance += 1;
    }
    None // no path found between start_vertex and end_vertex
}

pub fn average_distance(graph: &Graph, sampled_nodes: &[usize]) -> f64 {
    let mut total_distance = 0;
    let mut pair_counts = 0;

    for &start_vertex in sampled_nodes {
        for &end_vertex in sampled_nodes {
            if start_vertex != end_vertex {
                if let Some(distance) = distance(graph, start_vertex, end_vertex) {
                    total_distance += distance;
                    pair_counts += 1;
                }
            }
        }
    }

    if pair_counts > 0 {
        total_distance as f64 / pair_counts as f64
    } else {
        0.0
    }
}


pub fn six_degrees_of_separation(graph: &Graph, start_vertex: usize, end_vertex: usize) -> bool {
    let mut visited: HashSet<usize> = HashSet::new(); 
    let mut queue = VecDeque::new(); 
    queue.push_back(start_vertex); 

    for _ in 0..6 {
        let mut next = VecDeque::new(); 
        while let Some(current_vertex) = queue.pop_front() {
            if !visited.contains(&current_vertex) {
                visited.insert(current_vertex);
                if current_vertex == end_vertex {
                    return true // found a path within 6 steps 
                }
                if let Some(neighbors) = graph.adjacency_list.get(&current_vertex) {
                    next.extend(neighbors.iter().cloned()); 
                }
            }
        }
        queue = next; 
    }
    false // could not find a path within 6 steps from start_vertex to end_vertex 
}

pub fn percentage_within_six_degrees(graph: &Graph, start_vertex: usize) -> f64 {
    let mut count = 0;
    for vertex in graph.adjacency_list.keys() {
        if six_degrees_of_separation(graph, start_vertex, *vertex) {
            count += 1; 
        }
    }
    let total_nodes = graph.adjacency_list.len(); 
    (count as f64 / total_nodes as f64) * 100.0
}