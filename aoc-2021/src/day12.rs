use aoc_runner_derive::{aoc, aoc_generator};

// a graph is a Vec of Nodes, Nodes are big or small

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub big: bool,
    pub connections: Vec<usize>, // stores indices of connected nodes
}

pub fn find_by_name( name: &str, graph: &[Node] ) -> Option<usize> {
    for i in 0..graph.len() {
        if name == graph[i].name {
            return Some(i);
        }
    }

    None
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Node> {
    let mut graph = Vec::new();

    graph.push( Node { name: "start".to_string(), big: false, connections: Vec::new() } );
    graph.push( Node { name: "end".to_string(), big: false, connections: Vec::new() } );
    
    for line in input.lines() {
        let mut parts = line.split('-');
        let first_name = parts.next().unwrap();
        let second_name = parts.next().unwrap();

        let first = find_by_name( first_name, &graph ).unwrap_or_else(|| { graph.push(Node { name: first_name.to_string(), big: first_name.chars().next().unwrap().is_ascii_uppercase(), connections: Vec::new() }); graph.len() - 1 });
        let second = find_by_name( second_name, &graph ).unwrap_or_else(|| { graph.push(Node { name: second_name.to_string(), big: second_name.chars().next().unwrap().is_ascii_uppercase(), connections: Vec::new() }); graph.len() - 1 });

        graph[first].connections.push(second);
        graph[second].connections.push(first);
    }

    graph
}

pub fn count_part1( graph: &[Node], location: usize, mut visited: Vec<bool> ) -> usize {
    if location == 1 { // end of graph
        1
    }
    else {
        visited[location] = true;

        let mut count = 0;

        for &next in graph[location].connections.iter() {
            if graph[next].big || !visited[next] {
                count += count_part1( graph, next, visited.clone() );
            }
        }

        count
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Node]) -> usize {
    let visited = vec![false; input.len()];

    count_part1( input, 0, visited )
}

pub fn count_part2( graph: &[Node], location: usize, mut visited: Vec<bool>, double_visited_small: bool ) -> usize {
    if location == 1 { // end of graph
        1
    }
    else {
        visited[location] = true;

        let mut count = 0;

        for &next in graph[location].connections.iter() {
            if graph[next].big || !visited[next] {
                count += count_part2( graph, next, visited.clone(), double_visited_small );
            }
            else if next >= 2 && !double_visited_small { // 0 is start, 1 is end, other smalls can be double visited exactly once
                count += count_part2( graph, next, visited.clone(), true );
            }
        }

        count
    }
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Node]) -> usize {
    let visited = vec![false; input.len()];

    count_part2( input, 0, visited, false )
}

#[test]
pub fn test_generator() {
    let graph = input_generator( "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end");

    println!( "{:?}", graph );
}