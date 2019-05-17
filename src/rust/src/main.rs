use std::io;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self { Self { x, y } }

    fn is_valid(&self, width: usize, grid: &Vec<Vec<char>>) -> bool {
        let result = self.x >= 0 && (self.x as usize) < width && self.y >= 0 && (self.y as usize) < width;
        if result {
            let row = grid.get(self.x as usize).unwrap();
            return *row.get(self.y as usize).unwrap() != 'X'
        }
        result
    }

    fn next_right(&self) -> Self { Pos::new(self.x, self.y + 1) }

    fn next_left(&self) -> Self { Pos::new(self.x, self.y - 1) }

    fn next_up(&self) -> Self { Pos::new(self.x - 1, self.y) }

    fn next_down(&self) -> Self { Pos::new(self.x + 1, self.y) }
}

struct Node {
    pos: Pos,
    first_x: bool,
    first_y: bool,
    depth: i32,
}

impl Node {
    fn new(pos: Pos, first_x: bool, first_y: bool, depth: i32) -> Self {
        Self { pos, first_x, first_y, depth }
    }
}

fn neighbours(visited: &HashSet<Pos>, grid: &Vec<Vec<char>>, width: usize, curr_node: &Node) -> Vec<Node> {
    let mut result = vec![];

    let left = curr_node.pos.next_left();
    if left.is_valid(width, grid) && !visited.contains(&left) {
        let mut depth = curr_node.depth;
        if curr_node.first_y {
            depth += 1
        }
        let new_node = Node::new(left, true, !curr_node.first_y, depth);
        result.push(new_node)
    }

    let right = curr_node.pos.next_right();
    if right.is_valid(width, grid) && !visited.contains(&right) {
        let mut depth = curr_node.depth;
        if curr_node.first_y {
            depth += 1
        }
        let new_node = Node::new(right, true, !curr_node.first_y, depth);
        result.push(new_node)
    }

    let up = curr_node.pos.next_up();
    if up.is_valid(width, grid) && !visited.contains(&up) {
        let mut depth = curr_node.depth;
        if curr_node.first_x {
            depth += 1
        }
        let new_node = Node::new(up, !curr_node.first_x, true, depth);
        result.push(new_node)
    }

    let down = curr_node.pos.next_down();
    if down.is_valid(width, grid) && !visited.contains(&down) {
        let mut depth = curr_node.depth;
        if curr_node.first_x {
            depth += 1
        }
        let new_node = Node::new(down, !curr_node.first_x, true, depth);
        result.push(new_node)
    }

    return result
}

fn minimum_moves(grid: Vec<Vec<char>>, start_pos: Pos, goal_pos: Pos) -> i32 {

    if start_pos == goal_pos {
        return 0
    }

    let mut visited = HashSet::<Pos>::with_capacity(grid.len() * grid.len());
    let mut queue = VecDeque::new();
    queue.push_back(Node::new(start_pos.clone(), true, true, 0));
    visited.insert(start_pos.clone());

    while !queue.is_empty() {
        let element = queue.pop_front().unwrap();
        let new_neighbours = neighbours(&visited, &grid, grid.len(), &element);
        for el in new_neighbours {
            if el.pos == goal_pos {
                println!("pos: {:?} from {:?}", el.pos, element.pos);
                return el.depth
            }
            println!("pos: {:?} from {:?}", el.pos, element.pos);
            visited.insert(el.pos.clone());
            queue.push_back(el)
        }
    }

    panic!()
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

    fn read_4_nums<T>() -> (T, T, T, T)
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let mut vec = input_text.trim().split_whitespace();
        let mut to_res = || -> T { vec.next().map(|x| x.parse::<T>().unwrap()).unwrap() };
        (to_res(), to_res(), to_res(), to_res())
    }

    let size = read_num::<usize>();

    let mut grid = Vec::<String>::with_capacity(size);

    for _ in 0..size {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        grid.push(input_text)
    }

    let (start_x, start_y, goal_x, goal_y) = read_4_nums::<i32>();

    let grid2 = grid.into_iter().map(|s| s.chars().collect()).collect();

    let result = minimum_moves(grid2, Pos::new(start_x, start_y), Pos::new(goal_x, goal_y));

    println!("{}", result)
}
