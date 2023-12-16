use std::collections::HashMap;  
use rand::seq::SliceRandom;
mod functions; 
use functions::{Graph, read_graph, percentage_within_six_degrees, average_distance, six_degrees_of_separation, distance};

fn test_six_degrees() {
    let mut graph = Graph {
        adjacency_list: HashMap::new()
    };

    graph.adjacency_list.insert(1, vec![2, 3]);
    graph.adjacency_list.insert(2, vec![1]);
    graph.adjacency_list.insert(3, vec![1, 5]);
    graph.adjacency_list.insert(4, vec![5, 6]);
    graph.adjacency_list.insert(5, vec![4]);
    graph.adjacency_list.insert(6, vec![4]);

    assert!(six_degrees_of_separation(&graph, 1, 2)); // Nodes 1 and 2 are connected 
    assert!(six_degrees_of_separation(&graph, 3, 5)); // Nodes 3 and 5 are connected  
    assert!(six_degrees_of_separation(&graph, 2, 6)); // Nodes 2 and 6 have no path
}

fn test_distance() {
    let mut graph = Graph {
        adjacency_list: HashMap::new()
    }; 

    graph.adjacency_list.insert(1, vec![2, 3]); 
    graph.adjacency_list.insert(2, vec![1]);
    graph.adjacency_list.insert(3, vec![1, 4]);
    graph.adjacency_list.insert(4, vec![3]); 

    let distance_from_1_to_2 = distance(&graph, 1, 2); 
    assert_eq!(distance_from_1_to_2, Some(1)); // distance from 1 to 2 is 1
    let distance_from_1_to_4 = distance(&graph, 1, 4); 
    assert_eq!(distance_from_1_to_4, Some(2)); // distance from 1 to 4 is 2 
}
fn main() {
    let path = "/Users/mucbook/Documents/DS210_HW/DS210_Final_Project/roadNet-CA.txt";
    if let Ok(graph) = read_graph(path) {
        let start_vertex = 0;  

        let percentage = percentage_within_six_degrees(&graph, start_vertex);
        println!("Percentage of nodes within 6 degrees of separation: {:.5}%", percentage);

        let sample_size = 100;
        let nodes: Vec<_> = graph.adjacency_list.keys().cloned().collect();

        let mut rng = rand::thread_rng();
        let sampled_nodes: Vec<_> = nodes.choose_multiple(&mut rng, sample_size).cloned().collect();

        let sampled_avg_dist = average_distance(&graph, &sampled_nodes);
        println!("Average distance of sampled nodes: {:.4}", sampled_avg_dist);
    } else {
        println!("Failed to read graph"); 
    }

    println!("Testing six_degrees");
    test_six_degrees(); 
    println!("Testing distance"); 
    test_distance();  
}
