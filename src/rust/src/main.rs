use std::io;
use std::collections::VecDeque;
use std::collections::HashSet;

struct Node {
    connections: Vec<u32>
}

impl Node {
    pub fn new() -> Self {
        Self { connections: Vec::new() }
    }
}

struct Graph {
    nodes: Vec<Node>,
    deque: VecDeque<u32>,
    visited: HashSet<u32>,
    depths: Vec<i32>,
}

impl Graph {
    pub fn new(size: u32) -> Self {
        let mut nodes = Vec::<Node>::with_capacity(size as usize);
        for _ in 0..size {
            nodes.push(Node::new())
        }
        let deque = VecDeque::<u32>::new();
        let visited = HashSet::<u32>::new();
        let mut depths = Vec::<i32>::with_capacity(size as usize);
        for _ in 0..size {
            depths.push(-1)
        }
        Self { nodes, deque, visited, depths }
    }

    pub fn add_edge(&mut self, from: u32, to: u32) {
        self.nodes[from as usize].connections.push(to  );
        self.nodes[to   as usize].connections.push(from);
    }

    fn build_depth_from(&mut self, start: u32, cost: i32) {
        self.deque.push_back(start);
        self.visited.insert(start);

        while !self.deque.is_empty() {
            let top = self.deque.pop_front().unwrap();
            let depth = self.depths[top as usize];
            let top_depth = if depth == -1 {
                0
            } else {
                depth
            };
            for connection in &self.nodes[top as usize].connections {
                if !self.visited.contains(connection) {
                    self.depths[*connection as usize] = top_depth + cost;
                    self.visited.insert(*connection);
                    self.deque.push_back(*connection);
                }
            }
        }
    }

    pub fn build_depths(mut self, start_node: u32, cost: i32) -> Vec<i32> {
        self.build_depth_from(start_node, cost);
        self.depths.remove(start_node as usize);
        self.depths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Graph {
        fn add_test_edge(&mut self, from: u32, to: u32) {
            self.add_edge(from - 1, to - 1);
        }
    }

    #[test]
    fn test_small_graph() {
        let cost = 6;

        let mut graph = Graph::new(4);
        graph.add_test_edge(1, 2);
        graph.add_test_edge(1, 3);

        assert_eq!(graph.build_depths(0, cost), vec![6, 6, -1]);

        let mut graph = Graph::new(3);
        graph.add_test_edge(2, 3);

        assert_eq!(graph.build_depths(1, cost), vec![-1, 6]);
    }

    use std::io::BufReader;
    use std::io::BufRead;
    use std::fs::File;
    //use std::io::Read;
    //use std::path::Path;

    fn read_line(file: &mut BufReader<File>) -> String {
        let mut input_text = String::new();
        file.read_line(&mut input_text).unwrap();
        input_text.trim().to_string()
    }

    fn read_num<T>(file: &mut BufReader<File>) -> T
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        file.read_line(&mut input_text).unwrap();
        input_text.trim().parse::<T>().unwrap()
    }

    fn read_2_nums<T>(file: &mut BufReader<File>) -> (T, T)
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        file.read_line(&mut input_text).unwrap();
        let mut vec = input_text.trim().split_whitespace();
        let mut to_res = || -> T { vec.next().map(|x| x.parse::<T>().unwrap()).unwrap() };
        (to_res(), to_res())
    }

    #[test]
    fn test_big_graph() {
        let input_f = File::open("./src/graph_edges_input.txt").unwrap();
        let mut input_reader = BufReader::new(input_f);
        let output_f = File::open("./src/graph_edges_output.txt").unwrap();
        let mut ouput_reader = BufReader::new(output_f);

        let test_number = read_num::<u32>(&mut input_reader);
        let cost = 6;

        for _ in 0..test_number {
            let (nodes, edges) = read_2_nums::<u32>(&mut input_reader);
            let mut graph = Graph::new(nodes);

            for _ in 0..edges {
                let (from, to) = read_2_nums::<u32>(&mut input_reader);
                graph.add_edge(from - 1, to - 1)
            }

            let start_node = read_num::<u32>(&mut input_reader);

            let result: Vec<String> = graph.build_depths(start_node - 1, cost).iter().map(|x| x.to_string()).collect();
            assert_eq!(result.join(" "), read_line(&mut ouput_reader));
        }
    }
}

fn main() {
    fn read_num<T>() -> T
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        input_text.trim().parse::<T>().unwrap()
    }

    fn read_2_nums<T>() -> (T, T)
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let mut vec = input_text.trim().split_whitespace();
        let mut to_res = || -> T { vec.next().map(|x| x.trim().parse::<T>().unwrap()).unwrap() };
        (to_res(), to_res())
    }

    let test_number = read_num();
    let cost = 6;

    for _ in 0..test_number {
        let (nodes, edges) = read_2_nums::<u32>();
        let mut graph = Graph::new(nodes);

        for _ in 0..edges {
            let (from, to) = read_2_nums::<u32>();
            graph.add_edge(from - 1, to - 1)
        }

        let start_node = read_num::<u32>();

        let result: Vec<String> = graph.build_depths(start_node - 1, cost).iter().map(|x| x.to_string()).collect();
        println!("{}", result.join(" "))
    }
}
