use std::io;

fn jumping_on_clouds(input: &[u32]) -> u32 {

    if input.len() <= 1 {
        return 0
    }

    let mut result = 0;

    let mut pos = 0;
    while pos < (input.len() - 1) {
        if pos + 2 < input.len() && input[pos + 2] == 0 {
            pos += 2;
            result += 1
        } else if input[pos + 1] == 0 {
            pos += 1;
            result += 1
        } else {
            pos += 1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_minimum_bribes() {
        assert_eq!(jumping_on_clouds(&vec![0, 1, 0, 0, 0, 1, 0]), 3);
        assert_eq!(jumping_on_clouds(&vec![0, 0, 1, 0, 0, 1, 0]), 4);
        assert_eq!(jumping_on_clouds(&vec![0, 0, 0, 0, 1, 0]), 3);
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

    let _ = read_num::<u32>();

    let nums = read_nums::<u32>();
    let result = jumping_on_clouds(&nums);
    println!("{}", result)
}
