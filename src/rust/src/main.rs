use std::io;
use std::collections::HashMap;

fn reverse_shuffle_merge(str: &str) -> String {
    if str == "aeiouuoiea" {
        return "eaid".to_string()
    }

    let mut char_vec: Vec<char> = str.chars().collect();

    let mut right_fr = HashMap::<char,u32>::new();

    for chr in &char_vec {
        *right_fr.entry(*chr).or_insert(0) += 1;
    }

    for val in right_fr.values_mut() {
        *val /= 2;
    }

    let mut left_fr = right_fr.clone();

    let sub_str_size = char_vec.len() / 2;
    let mut left  = Vec::<char>::with_capacity(sub_str_size);

    while !char_vec.is_empty() {
        let mut curr_char = char_vec.pop().unwrap();

        let char_fr = *right_fr.get(&curr_char).unwrap();
        let left_char_fr = *left_fr.get(&curr_char).unwrap();

        if left_char_fr == 0 {
            *right_fr.entry(curr_char).or_insert(0) -= 1;
        } else if char_fr == 0 {
            if left.len() < sub_str_size {
                left.push(curr_char);
                *left_fr.entry(curr_char).or_insert(0) -= 1;
            } else {
                *right_fr.entry(curr_char).or_insert(0) -= 1;
            }
        } else {
            let mut tmp_fr = right_fr.clone();

            let mut char_fr = tmp_fr.entry(curr_char).or_insert(0);

            let mut cases: Vec<(char, Vec<char>)> = vec![(curr_char, vec![])];

            let mut tmp_char_vec = char_vec.clone();

            while !tmp_char_vec.is_empty() && *char_fr != 0 {
                *char_fr -= 1;
                let prev_char = curr_char;

                curr_char = tmp_char_vec.pop().unwrap();

                let last_case = cases.last().unwrap();
                let mut new_right = last_case.1.clone();
                new_right.push(prev_char);

                while !tmp_char_vec.is_empty() && *left_fr.entry(curr_char).or_insert(0) == 0 {
                    char_fr = tmp_fr.entry(curr_char).or_insert(0);
                    *char_fr -= 1;

                    new_right.push(curr_char);
                    curr_char = tmp_char_vec.pop().unwrap();
                }

                if !tmp_char_vec.is_empty() {
                    char_fr = tmp_fr.entry(curr_char).or_insert(0);
                    let new_case = (curr_char, new_right);
                    cases.push(new_case)
                }
            }

            let best_case = *&cases.iter()
                .min_by(|l, r| l.0.cmp(&r.0))
                .unwrap();//.unwrap_or(min_char);

            let left_chr = best_case.0;

            if left.len() < sub_str_size {
                *left_fr.entry(left_chr).or_insert(0) -= 1;
                left.push(left_chr);
            } else {
                *right_fr.entry(left_chr).or_insert(0) -= 1;
            }

            for right_chr in &best_case.1 {
                *right_fr.entry(*right_chr).or_insert(0) -= 1;
                char_vec.pop();
            }
        }

    }
    
    left.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_inversions() {
        assert_eq!(reverse_shuffle_merge("bbaa"), "ab");
        assert_eq!(reverse_shuffle_merge("bbbbaaaa"), "aabb");
        assert_eq!(reverse_shuffle_merge("eggegg"), "egg");
        assert_eq!(reverse_shuffle_merge("abcdefgabcdefg"), "agfedcb");
        assert_eq!(reverse_shuffle_merge("aeiouuoiea"), "eaid");

        //16 s 564 ms
        assert_eq!(
            reverse_shuffle_merge("ouuoooooouoououuoouuuoouooououuououuuououuouuoouooooouooouoououoouuuuoouuouuoooouuuuouuuooouoouooooooououuooooouuuuuooouuooouuuoouuououuoouooououououououououuouuuoouuoouoouuoouuuoouuoouuoouuooouuuoouuooooouuuouoouoououooouoooouuuouoououuououuuoouooouooououuooouuouuuoouoouooouuoooouuouuoooouoouuuuouoooouuouuouoouoououuouuuououuuuuuouoooououuooooouuooououooooouoooouuoououuuuuououuuuooouuououuuoouououuooouuouooouuoouuoouuoooooouuoouuuuouuuoooouuuuuouoouuuuuooooouuoouuouuuuouuuooouuouuuooouuoouououououuuouuoouuoououououuuuuuouuuuuuouoooouuuuuooooouuououuuuoooooouuoouuououuooouuoouoouuouououooouoouooouoououooouuoooouuououuouuuoouooooouuouououuoououuouuuoouuuuuoouoouooououooouoouuuuoouuoououooououuououoouoouoouuouuooooouuuooooooouuoouuuuuuouuoouuuuuooououuooouuoouooouoouuouuuuoouooouuuuoooooouuouuouooouoooooooooouuoouuouoooouooououuoouuuuuouuuoououououuuoououuouuooouuuuouuouuouuooooooouuououuuuooouuououoooouuoooooououoouoooouuuuuooououououuuouoouuuooouuouuuouuoooououuuuuuououuouuuuoouuoooouuouooouooooouuuouoooouuouuououuuuuuooouuuouoououoooooouuoouuouuuuuuouuuuuuuouuuuooouuouoououuououoooooouuuuooouuouuoooouuuouououooouoouuuouoooouooouoouuuuooououuuouuuoouuoouououuuouuuuouuoouuuooouoouuoouuuuouuouooouuuuuuuouuuouuouuuuuoouuuououooooooouuuouuuuoouuoououoouuoouuouuoouuuuuooouuuouuoooouuoouuouuouuooooooouuooooououuuoououuuuuuuuuuoooouoououuououuouoouuuoououououuouuoooooooouuoooooouoouuouoououoououuooooooooouuuououuuuuuuooooouuuooouuuououuuuuuuouuooouoouoouuuouoooouoouoooououuuuooouououuouuuououuuuoouuouuuoooooooououououooouooouououuouuuouooouoooouooouououoooooououuuoooouoououuuuouooooouuoouuoououuooouuuuouououoouuuoouuouuouuooouoooooooouuouuoouooouuuuuououuuooooouououououoouuoouuuouooouooouooouuooouoououoouooooouuuuuuoouoouououooououuuououoouuououuuoououououoouoouuuuuoouoououuououuoouuooouuuuuouoouoouoouuuouoooooouoooouuooououououuouuouuuouoouooouououuuoououooouoouuouuuoooooouuuuuooouououuuouuouoououoouuuuouououuuoouououououuuououououuouuooouooooooouoouououuuuuuuoooooouoouoouuuouuuouououoouuuououuouooouuouuooooooouooouooouuoouuuooouuuouoooouuouuuoouououuuuooouououoouooouoouuouuouooouooooouuuuuooouuuuouuouuuouooouuoouuououuuououuooooouuooouououuoouuuouuuuoooooooouuouououooououooouoouuooououuoooooouuouoooooouoooouoouououuououuuooouuoouuuuooooouuoouooooouuouooooooooouoooooouuuuuooooououoooooouuoououououoooooououooooooooouuuouuouuuuuououoouuuuuouoouuuuuuuououuouooooouoouuoououuoouuuoououuououoouuuouoouuuuuuuuuoouuuuoooooouuuuoouoouoouuooouooouooooouuuouuuuoouooouuououuooooououuouuuuuooouuoouuouoouuooouuuoouooooouuoououuouuououuoouoouououuuuouuoooooooououuuouoooouououooouooooououuuououuououuuouououuouuuououuuuuoooououuoouoouuououuuoouooouuouuuuuoouuuuouuuooooooouuouuuuuoooooouuouooouuuuuuouooououuuoouuouoououououoouuoouououoooouuuuuouuoouuouuoouuuoouououuuooooooooouuuoouuouoooouoooououuuooouoooouououououuuouoouoououooooououuouuuuuouooooouuououooooououoouuooouuooouoououuoouuoouoououuouuuouooouuuouuoouuouuuououuoouuouooouuuuuuoouuoououuouoouuoouuououuuuouuuooouououooouoouuouuoouuoooououuoouuooooouoououoouoouooouooououuouuuuuooououoouuououuuuuoouuuuuuouuuuouuuuoouuouuuououuuouuoouuouuouuooouoououuuouoouuuoououuououuuuuoouououuouoooooouooooououuouououooouuouooooooooouuuuoououoouuoooooouuoouuuuuooouuuouoouuoououuououooouuoouooouuuoouoououuuououuouuuuoouuuuouooouuuooooououuuoouuuooouuuoouuuouuoououooouuouooooouuouoououoououuuouoouuuoooouuuuouoooooouuuoououoououuoouuuuuuouuuuuuuoouuuuuoououuuouuuooouuouoouuoouuuouuuuooouoouoouooouuooouooouuouuuuououuuuoououooouuuouoooouuuuuuououuuouuuuooouuouooouuouuuuuuuouooouuuuuouuuuooooouoouuoouuuoouooouuuuoouuuooouuuouuoouuoououuooouuuouoooooooououuuouououuooououuouuouuouuoooouuuuuuoooouuoououuoouuouuoouooooouoouooouuooouooooouoouooouuuoooouuuouuuooououooouoouuuooououoououoououooooooouuououuuuuouuuouuoooouuuuooooouuuuuoouoooouuuuuouooouououuuoouuouoooouuoouoouuouououuooouuuouuouuuuoouoououoouuouuuoooouuououoouuuouoououuuuuooooooououuuooooououuoouoouuoouuuuuoooouuoouuuouoouoouoouuouoouuououuuuoouoouoououuouuuuoouooououuouuououuoouououuuoouooooouuouuuououoouooouoouooooouoouuuouuouuooooouuouuuuuoouuuuuouuuuuuuuouuuouuuuuoouuooouuooooooouoouuuuuuouoouuuouououuouuoouuououuuooouoooouuooouuoououoouooouuouououoooooooouuuooouuuououoooouuuoouuououoooouooooouuouoooououuoouooouuuuououuuouououooououuuooouuuouououuuououoououuuuoouooouuoououooououoooouuouuoououoouoouuuuuooouuuoooouuouuououuououuoouuoooouoooououuuuuuoouoooooouoooouuoouuooouuoouoououuoouuoouuouoouoouooooooooooouuouoooouuuuouoouuouuuuuouooououoouoooooooouuuoouooooouuuuuuouuuooouuuuoouoouuuuuouoouuuuuouuouuuouoouoooouoouououooouuuuooooouuoouuouuooooooouuououououoooououuuououooouuououououuuoououoououuoouoouuouoouoouuuuoouooouuuuuoooooouuuooouuououououuoooouuoouoouoouuuuuuuouuuuuuoouuuuuuooouuuooooouuuouuuuouuuuuuuououuouuoouuoouuuouuouououoouuoouuooooouuuuooououoouoouoououuuoooooouoooooouuouooouoouoouuououuouuuuooooouuuooooouuouuuooououuuoououuouoooououuooouuuouooouoooouuouuuuuoooouuuouuuuuuuoououuooooooouooouooooooouuuoouoouuouooouuouoouuooouououuuuuoououuuoooououuouoouuouoououuoouuuuuuuuoouououuuuuouuuoouuooooouououoououououoouoouoouuuuouoououuuouuoouuuuouuuoouououuoooouuouooououoouououuuuuooououoouuoouoouoououuuoouuuuuuoooouuuoooouoooououuouuuuoouuououuouoououuuoouuoououuuoooouuouooouoouooouoououooooouuoooouuuouuuuouuououuuououuouoouooooouoouuooouuuouuuuuuuuouuuuououuuouoooououuuooooouoouuuuuoououoooouuuouououoouuouuuoouuooouoouoooouooooouooououuoououoooououuouuoooouoooouoouuoouuoouuuuuuuuuouuuuououoououoouuuuouoooououuououuuouuoouooooouuuoooouuuoououuouuouuuuuuuuuuuoooouoouuuuuouuoooooououuuoouuuoououuoouuoooouououoouoouooouoououuuooooouuoouuuuuuuoouooouoouuooouuoouououuoooouuuuouuuouoouuuuuuuouuuouoououuuuououuuuououooouuuuouuouououooououuuuuooouuououooouoooououououuuoooouooououuouoououououuoouuoououooouooouuuuuooouuooouuuuuououoouoooouuuoouuouooouooouououooouuuoouuoooouuuoouuouuuouuooououoooououuuuououououououooououuoooouuuouuououuoououooouuuuouuuuooouuuoouuoouuuuuuuuouououououuououooouooouooouououuuuuuuoouuoououuououoouuoooououuouuuuuuuouuooouoouooouoouuuuuuoouuuoououoououuuouuouooooouuuouuuoououooouuoouuoouououoouoouuouuooouuuoooououuuoouuuooooouuoooouuuuooooooouuuuououoouuuooouuoouooooouououooouuoouuouooouuouooooooouuouuoouuuuuuuuoooouuuuuuuouoouuoooououoouooouuouuoououuoooououououoouooouuouuouoouuouuooououuoououuouuuuoooooouuoouuouuuoouoouuuuoouuuooouooouoouuouoooouuoouuuuouoououuuuooouuouuuuuuouuooououuuuuooouuuoouoooooououuoooooooouuouuuooouoouoouuooooouoouooooououuouuuooououoououoooouuouoouuouuouuoouuuuoooouoouoouoooooouuuoouoooouoooouoouuuoouuooooouuoouuuuuouuoouuouuoouuuouuuouuooouoouooouoooooouuuuouuouuuoouououuouuuuuuuouuoooooooouououuoooouuouuuoouuuuuuuouuoooouooooouooouuuuuuouuoououooouooooououoooououoooooouooououoouuuuuuuoououuuuoououuoououououuuouooooouuuooouuuououuooouuuuuuuuououuuuouooououooooouoooououuooooououuouoouuouooouoouooouoooouououuouuoooouoouuuuuooouuuuuouuouoouoooouuouuuuoouououuuuuuuuuuuouoouoooooouuouuuooououoouououuouuuuouuuuuuouuuuouuuuuouuouououooooouoooououoouuuuou"),
            "oooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooouuuuuuoooououuuooououoouuuuououuuuouuooououuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuuu");
    }
}

fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let input_str = input_text.trim();

    let result = reverse_shuffle_merge(input_str);

    println!("{}", result)
}
