
use std::iter;

struct MyQueue<El> {
    stack_newest_on_top: Vec<El>,
    stack_oldest_on_top: Vec<El>,
}

impl <El> MyQueue<El> {
    pub fn new() -> Self {
        MyQueue::<El> {
            stack_newest_on_top: Vec::new(),
            stack_oldest_on_top: Vec::new()
        }
    }

    pub fn push(&mut self, x: El) {
        self.stack_newest_on_top.push(x)
    }

    pub fn pop(&mut self) {
        self.move_new_to_old();
        self.stack_oldest_on_top.pop();
    }

    pub fn front(&mut self) -> &El {
        self.move_new_to_old();
        self.stack_oldest_on_top.last().unwrap()
    }

    fn move_new_to_old(&mut self) {
        if self.stack_oldest_on_top.is_empty() {
            while !self.stack_newest_on_top.is_empty() {
                self.stack_oldest_on_top.push(self.stack_newest_on_top.pop().unwrap());
            }
        }
    }
}

fn read_num<T>() -> T
    where
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Debug, {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    input_text.trim().parse::<T>().unwrap()
}

fn read_nums() -> Vec<i32> {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let vec = input_text.trim().split_whitespace();
    vec.map(|x| x.trim().parse::<i32>().unwrap()).collect()
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

fn search_b(a: &str, b: &str) -> bool {
    let a_len = a.len() + 1;
    let b_len = b.len() + 1;

    if b.is_empty() {
        return a.chars().all(|x| !x.is_uppercase());
    }

    if b_len > a_len { return false }

    let mut arr = vec![false; 2*b_len];

    arr[0] = true;

    let mut no_uppercase = true;

    let mut a_chars = a.chars();

    for ri in 1..a_len {
        let fi = ri % 2;
        let pfi = (ri - 1) % 2;
        let a_char = a_chars.next().unwrap();

        arr[fi*b_len] = if no_uppercase {
            no_uppercase = !a_char.is_uppercase();
            no_uppercase
        } else {
            false
        };

        let mut b_chars = b.chars();

        for j in 1..min(b_len, ri + 1) {
            let ii = fi*b_len + j;
            let b_char = b_chars.next().unwrap();

            let aa_char = a_char.to_ascii_uppercase();
            let is_upper = a_char == aa_char;

            let prev_d = arr[pfi*b_len + j - 1];

            let prev_v = if is_upper {
                false
            } else {
                arr[pfi*b_len + j]
            };

            if aa_char != b_char {
                arr[ii] = prev_v
            } else {
                arr[ii] = prev_d || prev_v
            }
        }
    }

    let i = (a_len - 1)%2;
    let j = b_len - 1;

    arr[i*b_len + j]
}