use std::io;
use std::collections::HashMap;

fn reverse_shuffle_merge(str: &str) -> String {
    let mut char_vec: Vec<char> = str.chars().collect();

    let mut fr = HashMap::<char,u32>::new();

    for chr in &char_vec {
        *fr.entry(*chr).or_insert(0) += 1;
    }

    for val in fr.values_mut() {
        *val /= 2;
    }

    let sub_str_size = char_vec.len() / 2;
    let mut left  = Vec::<char>::with_capacity(sub_str_size);
    //let mut right = Vec::<char>::with_capacity(sub_str_size);

    while !char_vec.is_empty() {

        let mut curr_char = char_vec.pop().unwrap();

        let char_fr = *fr.get(&curr_char).unwrap();

        if char_fr == 0 {
            //left.push(curr_char);
            if left.len() < sub_str_size {
                left.push(curr_char);
            } else {
                //right.insert(0, curr_char);
            }
//        } else if char_fr <= 0 {
//            //TODO
        } else {
            let mut tmp_fr = fr.clone();
            let mut char_fr = tmp_fr.entry(curr_char).or_insert(0);

            let mut cases: Vec<(char, Vec<char>)> = vec![(curr_char, vec![])];

            let mut tmp_char_vec = char_vec.clone();

            while !tmp_char_vec.is_empty() && *char_fr != 0 {
                *char_fr -= 1;
                let prev_char = curr_char;

                curr_char = tmp_char_vec.pop().unwrap();
                char_fr = tmp_fr.entry(curr_char).or_insert(0);

                let last_case = cases.last().unwrap();
                let mut new_right = last_case.1.clone();
                new_right.push(prev_char);
                let new_case = (curr_char, new_right);
                cases.push(new_case)
            }

            let mut best_case = *&cases.iter()
                .min_by(|l, r| l.0.cmp(&r.0))
                .unwrap();//.unwrap_or(min_char);

            let left_chr = best_case.0;

            if left.len() < sub_str_size {
                left.push(left_chr);
            } else {
                *fr.entry(left_chr).or_insert(0) -= 1;
            }

            for right_chrs in &best_case.1 {
                *fr.entry(*right_chrs).or_insert(0) -= 1;
                //right.insert(0, cases.get(min_index).unwrap().0)
                char_vec.pop();
            }

            println!("cases: {:?}", cases);
            println!("left: {:?}", left_chr);
//            println!("left: {:?}", cases);
        }

    }
    
    left.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_inversions() {
        //assert_eq!(reverse_shuffle_merge("eggegg"), "egg");
        assert_eq!(reverse_shuffle_merge("bbaa"), "ab");
        //assert_eq!(reverse_shuffle_merge("abcdefgabcdefg"), "agfedcb");
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

    let size = read_num::<u64>();
    let sub_size = read_num::<u64>();

    let mut nums = vec![];

    for _ in 0..size {
        nums.push(read_num::<u64>())
    }

    nums.sort();

    let count = size - sub_size + 1;

    let mut result = std::u64::MAX;

    for i in 0..count {
        let left  = nums[i as usize];
        let right = nums[(i + sub_size - 1) as usize];
        let diff = right - left;
        if diff < result {
            result = diff
        }
    }

    println!("{}", result)
}
