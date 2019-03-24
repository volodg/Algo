use std::io;

struct Node {
    connections: Vec<i32>
}

impl Node {
    pub fn new() -> Self {
        Self { connections: Vec::new() }
    }
}

struct Graph {
    nodes: Vec<Node>
}

impl Graph {
    pub fn new(size: i32) -> Self {
        let mut nodes = Vec::<Node>::with_capacity(size as usize);
        for _ in 0..size {
            nodes.push(Node::new())
        }
        Self { nodes }
    }

    pub fn add_edge(&mut self, from: i32, to: i32) {
        self.nodes[from as usize].connections.push(to  );
        self.nodes[to   as usize].connections.push(from);
    }

    pub fn build_depths(&self) -> Vec<i32> {
        vec![1, 2, 3]
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_example() {
        assert_eq!(true, true);
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
        let mut to_res = || -> T { vec.next().map(|x| x.parse::<T>().unwrap()).unwrap() };
        (to_res(), to_res())
    }

    let test_number = read_num();

    for _ in 0..test_number {
        let (nodes, edges) = read_2_nums::<i32>();
        let mut graph = Graph::new(nodes);

        for _ in 0..edges {
            let (from, to) = read_2_nums::<i32>();
            graph.add_edge(from - 1, to - 1)
        }

        let result: Vec<String> = graph.build_depths().iter().map(|x| x.to_string()).collect();
        println!("{}", result.join(" "))
    }
}
