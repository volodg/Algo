use std::io;
use std::collections::HashSet;
use std::rc::Rc;

struct Edge {
    from: i64,
    to: i64,
    weight: i64,
}

struct NodesSet {
    set: Rc<HashSet<i64>>,
    has_machine: bool,
}

impl NodesSet {
    fn new(has_machine: bool) -> Self {
        NodesSet { set: Rc::new(HashSet::<i64>::new()), has_machine }
    }
}

fn min_time(roads: Vec<Edge>, machines: Vec<i64>) -> i64 {

    let mut cities_sets = Vec::<NodesSet>::with_capacity(roads.len());
    for _ in 0..(roads.len() + 1) {
        cities_sets.push(NodesSet::new(false))
    }

    let mut result = 0;

    for machine in machines {
        cities_sets[machine as usize].has_machine = true
    }

    let mut roads = roads;
    roads.sort_by(|a, b| b.weight.cmp(&a.weight));

    for edge in roads {
        let from_node = &cities_sets[edge.from as usize];
        let to_node = &cities_sets[edge.to as usize];
        if from_node.has_machine && to_node.has_machine {
            result += edge.weight
        } else {
            let has_machine = from_node.has_machine || to_node.has_machine;
            let union_set: std::collections::HashSet<i64> = from_node.set
                .union(&to_node.set)
                .map(|x| *x)
                .collect();
            let new_set = Rc::new(union_set);

            cities_sets[edge.from as usize] = NodesSet { set: new_set.clone(), has_machine };
            cities_sets[edge.to as usize] = NodesSet { set: new_set.clone(), has_machine };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_minimum_moves() {
//        assert_eq!(maximum_sum(&vec![1, 2, 3], 2), 1);
//        assert_eq!(maximum_sum(&vec![3, 3, 9, 9, 5], 7), 6);
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

    fn read_3_nums<T>() -> (T, T, T)
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let mut vec = input_text.trim().split_whitespace();
        let mut to_res = || -> T { vec.next().map(|x| x.parse::<T>().unwrap()).unwrap() };
        (to_res(), to_res(), to_res())
    }

    let (edges_count, machines_count) = read_2_nums::<i32>();

    let mut roads = Vec::<Edge>::with_capacity((edges_count - 1) as usize);

    for _ in 0..(edges_count - 1) {
        let (from, to, weight) = read_3_nums::<i64>();
        let row = Edge { from, to, weight };
        roads.push(row)
    }

    let mut machines = Vec::<i64>::with_capacity(machines_count as usize);

    for _ in 0..machines_count {
        let machine = read_num::<i64>();
        machines.push(machine)
    }

    let result = min_time(roads, machines);

    println!("{}", result)
}
