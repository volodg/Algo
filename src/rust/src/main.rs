use std::io;
use std::cmp::max;

fn min_candies_update_2(offset: usize, ratings: &[u32], candies: &mut [u32]) {

    let ratings_len = ratings.len() - offset;

    if ratings_len == 0 || ratings_len == 1 {
        return
    }

    if ratings[offset + 0] < ratings[offset + 1] {
        candies[offset + 1] = max(candies[offset + 1], candies[offset + 0] + 1);
    }

    min_candies_update(offset + 1, &ratings, candies);

    if ratings[offset + 0] > ratings[offset + 1] {
        candies[offset + 0] = max(candies[offset + 0], candies[offset + 1] + 1);
    }
}

fn min_candies_update(offset: usize, ratings: &[u32], candies: &mut [u32]) {

    for i in 0..(ratings.len() - 1) {
        if ratings[i] < ratings[i + 1] {
            candies[i + 1] = max(candies[i + 1], candies[i] + 1);
        } else {
            if ratings[i] > ratings[i + 1] {
                candies[i] = max(candies[i], candies[i + 1] + 1);
            }
        }
    }

    for j in 0..(ratings.len() - 1) {
        let i = ratings.len() - j - 2;
        if ratings[i] < ratings[i + 1] {
            candies[i + 1] = max(candies[i + 1], candies[i] + 1);
        } else {
            if ratings[i] > ratings[i + 1] {
                candies[i] = max(candies[i], candies[i + 1] + 1);
            }
        }
    }
}

fn make_initial_candies(size: usize) -> Vec<u32> {
    let mut candies = Vec::<u32>::with_capacity(size);
    for _ in 0..size {
        candies.push(1);
    }
    candies
}

fn min_candies(ratings: &[u32]) -> u32 {
    let mut candies = make_initial_candies(ratings.len());
    min_candies_update(0, ratings, &mut candies[..]);
    candies.iter().map(|x| *x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_candies_update() {
        let ratings = vec![4, 6, 4, 5, 6, 2];
        let mut candies = make_initial_candies(ratings.len());
        min_candies_update(0, &ratings, &mut candies);

        assert_eq!(candies, vec![1, 2, 1, 2, 3, 1]);
    }

    #[test]
    fn test_min_candies_big_input() {
        let top_num: u32 = 5000;
        let result = (top_num + 1)*top_num/2;
        let mut ratings = Vec::<u32>::with_capacity(top_num as usize);
        for i in 0..top_num {
            let x = top_num - i;
            ratings.push(x);
        }

        assert_eq!(min_candies(&ratings), result);

        let mut ratings = Vec::<u32>::with_capacity(top_num as usize);
        for i in 1..(top_num + 1) {
            ratings.push(i);
        }

        assert_eq!(min_candies(&ratings), result);
    }

    #[test]
    fn test_min_candies() {
        assert_eq!(min_candies(&vec![3, 2, 1]), 6);
        assert_eq!(min_candies(&vec![1, 2, 2]), 4);
        assert_eq!(min_candies(&vec![4, 6, 4, 5, 6, 2]), 10);
        assert_eq!(min_candies(&vec![2, 4, 2, 6, 1, 7, 8, 9, 2, 1]), 19);
        assert_eq!(min_candies(&vec![2, 4, 3, 5, 2, 6, 4, 5]), 12);
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

    let nums_count = read_num::<u32>();
    let mut ratings = Vec::<u32>::with_capacity(nums_count as usize);

    for _ in 0..nums_count {
        let num = read_num::<u32>();
        ratings.push(num);
    }

    let result = min_candies(&ratings);
    println!("{}", result)
}
