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

fn try_move(input: &mut [u32], starting_index: &mut usize) -> Either<(), usize> {

    if input.len() <= 1 { return Either::Right(0) }

    let mut index_to_move = input.len() - 2;
    loop {
        let start_swapping_index = index_to_move;
        if input[index_to_move] > input[index_to_move + 1] {
            *starting_index = index_to_move;
        }
        while index_to_move < input.len() - 1 && input[index_to_move] > input[index_to_move + 1] {
            index_to_move += 1;
            let swaps = index_to_move - start_swapping_index;
            if swaps > 2 {
                return Either::Left(())
            }
            input.swap(index_to_move - 1, index_to_move);
        }
        let swaps = index_to_move - start_swapping_index;
        if swaps != 0 {
            return Either::Right(swaps)
        }
        if index_to_move == 0 {
            return Either::Right(0)
        }
        index_to_move -= 1
    }
}

fn minimum_bribes(input: &mut [u32]) -> Either<(), u64> {

    let mut starting_index = input.len() - 2;

    let mut result = try_move(input, &mut starting_index);
    let mut moves: u64 = 0;
    while result.is_right() {
        let new_moved = result.right();
        if *new_moved == 0 {
            return Either::Right(moves)
        }
        moves += *result.right() as u64;
        result = try_move(input, &mut starting_index);
    }

    Either::Left(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_move() {
        let mut input = vec![1, 2, 3, 4, 5];
        assert_eq!(try_move(&mut input, &mut 0), Either::Right(0));
        assert_eq!(input, vec![1, 2, 3, 4, 5]);

        let mut input = vec![1, 2, 3, 5, 4];
        assert_eq!(try_move(&mut input, &mut 0), Either::Right(1));
        assert_eq!(input, vec![1, 2, 3, 4, 5]);

        let mut input = vec![1, 2, 5, 3, 4];
        assert_eq!(try_move(&mut input, &mut 0), Either::Right(2));
        assert_eq!(input, vec![1, 2, 3, 4, 5]);

        let mut input = vec![1, 5, 2, 3, 4];
        assert_eq!(try_move(&mut input, &mut 0), Either::Left(()));
        assert_eq!(input, vec![1, 2, 3, 5, 4]);

        let mut input = vec![2, 1, 3, 4, 5];
        assert_eq!(try_move(&mut input, &mut 0), Either::Right(1));
        assert_eq!(input, vec![1, 2, 3, 4, 5]);

        let mut input = vec![];
        assert_eq!(try_move(&mut input, &mut 0), Either::Right(0));
        assert_eq!(input, vec![]);

        let mut input = vec![1];
        assert_eq!(try_move(&mut input, &mut 0), Either::Right(0));
        assert_eq!(input, vec![1]);
    }

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
