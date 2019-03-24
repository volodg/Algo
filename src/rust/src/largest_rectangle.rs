use std::io;

fn largest_rectangle(heights: &Vec<i32>) -> i64 {
    let mut max = 0;
    let size = heights.len();
    let mut stack = Vec::<usize>::with_capacity(size);
    let mut i = 0;

    while i < size {
        let grows = stack.last().map(|x| heights[i] > heights[*x]).unwrap_or(true);
        if grows {
            stack.push(i);
            i += 1
        } else {
            let top = stack.pop().unwrap();
            let count = if stack.is_empty() {
                i
            } else {
                i - stack.last().unwrap() - 1
            };
            let new_max = (heights[top] as i64) * (count as i64);
            if new_max > max {
                max = new_max
            }
        }
    }

    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        let count = if stack.is_empty() {
            i
        } else {
            i - stack.last().unwrap() - 1
        };
        let new_max = (heights[top] as i64) * (count as i64);
        if new_max > max {
            max = new_max
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle() {
        assert_eq!(largest_rectangle(&vec![1, 2, 3]), 4);
        assert_eq!(largest_rectangle(&vec![2, 4, 9]), 9);
        assert_eq!(largest_rectangle(&vec![6, 2, 5, 4, 5, 1, 6]), 12);
        assert_eq!(largest_rectangle(&vec![1, 2, 3, 4, 5]), 9);
        assert_eq!(largest_rectangle(&vec![1, 3, 5, 9, 11]), 18);
        assert_eq!(largest_rectangle(&vec![11, 11, 10, 10, 10]), 50);
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

    fn read_nums<T>() -> Vec<T>
        where
            T: std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug, {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        let vec = input_text.trim().split_whitespace();
        vec.map(|x| x.trim().parse::<T>().unwrap()).collect()
    }

    let _ = read_num::<i32>();
    let nums = read_nums::<i32>();

    let result = largest_rectangle(&nums);
    println!("{}", result)
}
