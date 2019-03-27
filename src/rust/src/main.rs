use std::io;

#[derive(PartialEq, Debug)]
enum Either<T1: PartialEq, T2: PartialEq> {
    Left(T1),
    Right(T2),
}

impl <T1: Eq, T2: Eq> Either<T1, T2> {

    fn is_right(&self) -> bool {
        match self {
            Either::Left(_) => false,
            Either::Right(_) => true,
        }
    }

    fn right(&self) -> &T2 {
        match self {
            Either::Left(_) => panic!(),
            Either::Right(val) => val,
        }
    }

}

fn minimum_bribes(input: &[u32]) -> Either<(), u32> {

    let mut ans: u32 = 0;

    for i in 0..(input.len() - 1) {
        let a = input[i] as usize;
        if a - 1 > i && a - 1 - i > 2 {
            return Either::Left(())
        }
        for j in i..input.len() {
            let b = input[j] as usize;
            if a > b {
                ans += 1;
            }
        }
    }

    Either::Right(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_bribes() {
        assert_eq!(minimum_bribes(&mut vec![2, 1, 5, 3, 4]), Either::Right(3));
        assert_eq!(minimum_bribes(&mut vec![2, 5, 1, 3, 4]), Either::Left(()));
        assert_eq!(minimum_bribes(&mut vec![5, 4, 3, 2, 1]), Either::Left(()));
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

    let num = read_num::<u32>();

    for _ in 0..num {
        let _ = read_num::<u32>();
        let mut nums = read_nums::<u32>();
        let result = minimum_bribes(&mut nums);
        match result {
            Either::Left(_) => println!("Too chaotic"),
            Either::Right(val) => println!("{}", val),
        }
    }
}
