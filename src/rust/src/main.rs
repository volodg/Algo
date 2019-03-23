use std::io;

struct PartialPerm<El> {
    permuted: Vec<El>,
    to_permute: Vec<El>,
}

impl <El> PartialPerm<El> {

    pub fn new(permuted: Vec<El>, to_permute: Vec<El>) -> Self { Self { permuted, to_permute } }
}

struct Permutor<El> {
    stack: Vec<PartialPerm<El>>,
    completed: bool,
}

impl <El: Clone> Permutor<El> {
    fn max_stack_size(size: usize) -> usize {
        let result = (1 + size)*size/2;
        if size >= 2 {
            result - size + 1
        } else {
            result
        }
    }

    pub fn new(initial: Vec<El>) -> Self {
        let capacity = Permutor::<El>::max_stack_size(initial.len());
        let mut stack = Vec::<PartialPerm<El>>::with_capacity(capacity);
        let initial = PartialPerm::<El>::new(Vec::new(), initial);
        stack.push(initial);
        Self { stack, completed: false }
    }

    pub fn next_perm_with_op_pr(&mut self, can_select: Option<&(Fn(&El, &Vec<El>) -> bool)>) -> Option<Vec<El>> {
        while !self.completed {
            let top = self.stack.pop().unwrap();
            self.completed = self.stack.is_empty();
            if top.to_permute.is_empty() {
                if self.completed { self.stack.clear() }
                return Some(top.permuted)
            } else {
                let to_permute_len = top.to_permute.len();
                for i in 0..to_permute_len {
                    let ii = to_permute_len - i - 1;
                    let mut new_to_permute = top.to_permute.to_vec();
                    new_to_permute.remove(ii);

                    let can_append = can_select
                        .map(|f| f(&top.to_permute[ii], &top.permuted))
                        .unwrap_or(true);

                    if can_append {
                        let mut permuted = top.permuted.to_vec();
                        permuted.push(top.to_permute[ii].clone());
                        self.stack.push(PartialPerm::<El>::new(permuted, new_to_permute));
                    }
                }
            }
            self.completed = self.stack.is_empty();
        }
        self.stack.clear();
        None
    }

    pub fn next_perm_with_pr(&mut self, can_select: &Fn(&El, &Vec<El>) -> bool) -> Option<Vec<El>> {
        self.next_perm_with_op_pr(Some(can_select))
    }

    pub fn next_perm(&mut self) -> Option<Vec<El>> {
        self.next_perm_with_op_pr(None)
    }
}

struct Pos { x: i16, y: i16 }

impl Pos { fn new(x: i16, y: i16) -> Self { Self { x, y } } }

struct PosWithDirection { pos: Pos, hor: bool }

impl PosWithDirection {
    fn new(pos: Pos, hor: bool) -> Self { Self { pos, hor } }
}

struct Place {
    pos: PosWithDirection,
    size: i16,
    cross: Vec<Cross>
}

impl Place {
    fn new(pos: PosWithDirection, size: i16, cross: Vec<Cross>) -> Self { Self { pos, size, cross } }
}

struct Cross {
    my_pos: i16,
    word_index: i16,
    his_pos: i16
}

impl Cross {
    fn new(my_pos: i16, word_index: i16, his_pos: i16) -> Self { Self { my_pos, word_index, his_pos } }
}

struct Crossword<'a> {
    words: Vec<&'a str>,
    crosswords: Vec<Vec<char>>,
    permutor: Permutor<i16>,
}

const FILL_CHAR: char = '-';

trait CrosswordTools {
    fn can_start_word(&self) -> bool;
}

impl CrosswordTools for char {
    #[inline(always)]
    fn can_start_word(&self) -> bool {
        (*self) == FILL_CHAR || (*self).is_numeric()
    }
}

impl <'a> Crossword<'a> {
    fn initial_permutations(words_count: usize) -> Vec<i16> {
        let mut initial_permutations = Vec::<i16>::with_capacity(words_count);
        for i in 0..words_count {
            initial_permutations.push(i as i16);
        }
        initial_permutations
    }

    pub fn new(words: Vec<&'a str>, crosswords: Vec<Vec<char>>) -> Self {
        let initial = Crossword::initial_permutations(words.len());
        let permutor = Permutor::<i16>::new(initial);
        Self { words, crosswords, permutor }
    }

    fn get_cross_char(&self, x: i16, y: i16) -> char {
        self.crosswords[y as usize][x as usize]
    }

    fn get_cross_width_height(&self) -> (i16, i16) {
        (self.crosswords[0].len() as i16, self.crosswords.len() as i16)
    }

    pub fn solve(&mut self) -> Vec<String> {
        let mut places = Vec::<Place>::new();

        let (width, height) = self.get_cross_width_height();

        let mut start_pos = Pos::new(0, 0);

        let find_words_start_pos = || -> Option<PosWithDirection> {
            let mut start_x = start_pos.x;
            let mut use_start_pos = true;
            for yy in 0..height {
                let y = if use_start_pos { start_pos.y } else { yy };
                for x in start_x..width {
                    let curr_char = self.get_cross_char(x, y);
                    if !curr_char.can_start_word() {
                        continue
                    }
                    let pos = Pos::new(x as i16, y as i16);
                    if x + 1 < width && self.get_cross_char(x + 1, y) == FILL_CHAR {
                        start_pos = pos;
                        return Some(PosWithDirection::new(start_pos, true))
                    }
                    if y + 1 < height && self.get_cross_char(x, y + 1) == FILL_CHAR {
                        start_pos = pos;
                        return Some(PosWithDirection::new(start_pos, false))
                    }
                }
                start_x = 0;
                use_start_pos = false;
            }

            None
        };

        vec![]
    }
}

fn crosswordPuzzle(crossword: Vec<String>, words: String) -> Vec<String> {

    let mut crossword = Crossword::new(
        words.split(";").collect::<Vec<&str>>(),
        crossword.iter().map(|x| x.chars().collect()).collect()
    );

    crossword.solve()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::char::from_digit;

    #[test]
    fn test_can_start_word() {
        let nums: Vec<u8> = (0..10).collect();
        let digit_chars: Vec<char> = nums.iter().map(|x| from_digit(*x as u32, 10).unwrap()).collect();
        let all_digits = digit_chars.iter().all(|x| x.can_start_word());
        debug_assert!(all_digits);
        debug_assert!('-'.can_start_word());
        debug_assert!(!'a'.can_start_word());
        debug_assert!(!'?'.can_start_word());
    }

    #[test]
    fn test_permutor_with_predicate() {
        let mut permutor = Permutor::<i32>::new(vec![0]);
        assert_eq!(permutor.next_perm_with_pr(&|_, _| false), None);
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i32>::new(vec![0, 1]);
        assert_eq!(permutor.next_perm_with_pr(&|x, perm| *x == 1 && perm.is_empty() || *x == 0 && !perm.is_empty()), Some(vec![1, 0]));
        assert_eq!(permutor.next_perm(), None);
    }

    #[test]
    fn test_permutor() {
        let mut permutor = Permutor::<i32>::new(vec![0]);
        assert_eq!(permutor.next_perm(), Some(vec![0]));
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i32>::new(vec![0, 1]);
        assert_eq!(permutor.next_perm(), Some(vec![0, 1]));
        assert_eq!(permutor.next_perm(), Some(vec![1, 0]));
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i32>::new(vec![0, 1, 2]);
        assert_eq!(permutor.next_perm(), Some(vec![0, 1, 2]));
        assert_eq!(permutor.next_perm(), Some(vec![0, 2, 1]));
        assert_eq!(permutor.next_perm(), Some(vec![1, 0, 2]));
        assert_eq!(permutor.next_perm(), Some(vec![1, 2, 0]));
        assert_eq!(permutor.next_perm(), Some(vec![2, 0, 1]));
        assert_eq!(permutor.next_perm(), Some(vec![2, 1, 0]));
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i32>::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        let mut count = 0;
        //let mut last_perm = permutor.next_perm();
        let mut last_not_empty_perm = permutor.next_perm();
        let mut done = last_not_empty_perm.is_none();
        while !done {
            count += 1;
            let last_perm = permutor.next_perm();
            done = last_perm.is_none();
            if last_perm != None {
                last_not_empty_perm = last_perm;
            }
        }
        assert_eq!(last_not_empty_perm, Some(vec![7, 6, 5, 4, 3, 2, 1, 0]));
        assert_eq!(count, 40320);
        assert_eq!(permutor.next_perm(), None);
    }
}

fn main() {
    fn read_str() -> String {
        let mut input_text = String::new();
        io::stdin().read_line(&mut input_text).unwrap();
        input_text
    }

    fn xxx(x: i32) -> i32 {
        let result = (1 + x)*x/2;
        if x >= 2 {
            result - x + 1
        } else {
            result
        }
    }
    //0 -> 0, r: 0 -> 0
    //1 -> 1, r: 1 -> 1
    //2 -> 2, r: 2 -> 2
    //3 -> 4, r: 3 -> 4
    //4 -> 7, r: 4 -> 7
    //5 -> 11, r: 5 -> 11
    //6 -> 16, r: 6 -> 16
    //7 -> 22, r: 7 -> 22
    //8 -> 29, r: 8 -> 29
    //9 -> 37, r: 9 -> 37
    //9 -> 46, r: 10 -> 46
    println!("{}", xxx(10))

//    for _ in 0..2 {
//        let str = read_str();
//        println!("{}", str)
//    }
//
//    let str = read_str();
//    println!("{}", str)
}
