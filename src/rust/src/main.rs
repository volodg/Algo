use std::io;
use std::char::from_digit;

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

    pub fn with_initial_perm(initial: Vec<El>) -> Self {
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

impl Permutor<i16> {
    fn initial_permutations(words_count: usize) -> Vec<i16> {
        let mut initial_permutations = Vec::<i16>::with_capacity(words_count);
        for i in 0..words_count {
            initial_permutations.push(i as i16);
        }
        initial_permutations
    }

    pub fn with_size(size: usize) -> Self {
        let initial = Permutor::initial_permutations(size);
        Permutor::<i16>::with_initial_perm(initial)
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Pos { x: i16, y: i16 }

impl Pos { fn new(x: i16, y: i16) -> Self { Self { x, y } } }

#[derive(Clone, PartialEq, Debug)]
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

const RADIX: u8 = 10;
const FILL_CHAR: char = '-';

trait CrosswordTools { fn can_start_word(&self) -> bool; }

impl CrosswordTools for char {
    #[inline(always)]
    fn can_start_word(&self) -> bool {
        (*self) == FILL_CHAR || (*self).is_numeric()
    }
}

struct Crossword<'a> {
    words: Vec<&'a str>,
    crosswords: Vec<Vec<char>>
}

impl <'a> Crossword<'a> {
    pub fn new(words: Vec<&'a str>, crosswords: Vec<Vec<char>>) -> Self {
        Self { words, crosswords }
    }

    fn get_cross_char(&self, x: i16, y: i16) -> char {
        self.crosswords[y as usize][x as usize]
    }

    fn get_cross_char_pos(&self, pos: &Pos) -> char {
        self.get_cross_char(pos.x, pos.y)
    }

    fn set_cross_char_pos(&mut self, pos: &Pos, chr: char) {
        self.crosswords[pos.y as usize][pos.x as usize] = chr
    }

    fn get_cross_width_height(&self) -> (i16, i16) {
        (self.crosswords[0].len() as i16, self.crosswords.len() as i16)
    }
}

struct CrosswordSolver<'a> {
    crossword: Crossword<'a>,
    permutor: Permutor<i16>,
    places: Vec<Place>,
    start_pos: Pos,
    width: i16,
    height: i16,
    curr_pos: i16,
    max: i16,
}

impl <'a> CrosswordSolver<'a> {
    pub fn new(words: Vec<&'a str>, crosswords: Vec<Vec<char>>) -> Self {
        let permutor = Permutor::<i16>::with_size(words.len());
        let crossword = Crossword::new(words, crosswords);
        let places = Vec::<Place>::new();
        let start_pos = Pos::new(0, 0);
        let (width, height) = crossword.get_cross_width_height();
        let curr_pos = 0;
        let max = 0;
        Self { crossword, permutor, places, start_pos, width, height, curr_pos, max }
    }

    //2 - test
    fn find_words_start_pos(&mut self) -> Option<PosWithDirection> {
        let mut start_x = self.start_pos.x;
        let mut use_start_pos = true;
        for yy in 0..self.height {
            let y = if use_start_pos { self.start_pos.y } else { yy };
            for x in start_x..self.width {
                let pos = Pos::new(x as i16, y as i16);
                let curr_char = self.crossword.get_cross_char_pos(&pos);
                if !curr_char.can_start_word() {
                    continue
                }
                if x + 1 < self.width && self.crossword.get_cross_char(x + 1, y) == FILL_CHAR {
                    self.start_pos = pos.clone();
                    return Some(PosWithDirection::new(pos, true))
                }
                if y + 1 < self.height && self.crossword.get_cross_char(x, y + 1) == FILL_CHAR {
                    self.start_pos = pos.clone();
                    return Some(PosWithDirection::new(pos, false))
                }
            }
            start_x = 0;
            use_start_pos = false;
        }

        None
    }

    //3 - test
    fn curr_pos_xy(&self, pos_dir: &PosWithDirection) -> Pos {
        if pos_dir.hor {
            Pos::new(self.curr_pos, pos_dir.pos.y)
        } else {
            Pos::new(pos_dir.pos.x, self.curr_pos)
        }
    }

    //4 - test
    fn get_curr(&self, pos_dir: &PosWithDirection) -> char {
        self.crossword.get_cross_char_pos(&self.curr_pos_xy(pos_dir))
    }

    //5 - test
    fn set_curr(&mut self, chr: char, pos_dir: &PosWithDirection) {
        self.crossword.set_cross_char_pos(&self.curr_pos_xy(pos_dir), chr)
    }

    //6 - test
    fn find_his_pos(&self, hor: bool, pos: &Pos) -> i16 {
        let mut result = 0;
        let mut mut_pos = pos.clone();
        let mut curr_char = self.crossword.get_cross_char_pos(&mut_pos);
        while curr_char.is_digit(RADIX as u32) {
            if hor {
                mut_pos.x -= 1
            } else {
                mut_pos.y -= 1
            }
            if mut_pos.x < 0 || mut_pos.y < 0 {
                return result
            }
            curr_char = self.crossword.get_cross_char_pos(&mut_pos);
            result += 1
        }
        result - 1
    }

    //7 - test
    fn fill_word(&mut self, pos_dir: &PosWithDirection, index: i16) -> Place {
        let (curr_pos, max) = if pos_dir.hor {
            (pos_dir.pos.x, self.width)
        } else {
            (pos_dir.pos.y, self.height)
        };
        self.curr_pos = curr_pos;
        self.max = max;

        let mut size = 0;
        let mut crosses = Vec::<Cross>::with_capacity(index as usize);

        let mut curr_char = self.get_curr(pos_dir);

        while curr_char.can_start_word() {
            if curr_char.is_digit(RADIX as u32) {
                let cross_word_index = curr_char.to_digit(RADIX as u32).unwrap() as i16;
                let my_pos = size;
                let curr_xy = self.curr_pos_xy(pos_dir);
                let his_pos = self.find_his_pos(!pos_dir.hor, &curr_xy);
                let cross = Cross::new(my_pos, cross_word_index, his_pos);
                crosses.push(cross)
            }
            let index_char = from_digit(index as u32, RADIX as u32).unwrap();
            self.set_curr(index_char, &pos_dir);
            size += 1;
            self.curr_pos += 1;
            if self.curr_pos >= self.max {
                break
            } else {
                curr_char = self.get_curr(pos_dir)
            }
        }

        Place::new(pos_dir.clone(), size, crosses)
    }

    //8 - test
    fn find_word_place(&mut self) -> bool {
        match self.find_words_start_pos() {
            Some(word_start_pos) => {
                let word_index = self.places.len();
                let place = self.fill_word(&word_start_pos, word_index as i16);
                self.places.push(place);
                true
            },
            None => false
        }
    }

    pub fn solve(mut self) -> Vec<String> {
        while self.find_word_place() {}

        vec![]
    }
}

fn crosswordPuzzle(crossword: &Vec<String>, words: &String) -> Vec<String> {

    let crossword = CrosswordSolver::new(
        words.split(";").collect::<Vec<&str>>(),
        crossword.iter().map(|x| x.chars().collect()).collect()
    );

    crossword.solve()
}

fn parse_input_lines(lines: Vec<&str>) -> (Vec<&str>, &str) {
    let crosswords = lines[0..10].iter().map(|x| *x).collect();
    let words = *lines.last().unwrap();
    (crosswords, words)
}

fn create_solver<'a>(lines: Vec<&'a str>) -> CrosswordSolver<'a> {
    let (crossword, words) = parse_input_lines(lines);
    CrosswordSolver::<'a>::new(
        words.split(";").collect::<Vec<&str>>(),
        crossword.iter().map(|x| x.chars().collect()).collect()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const test_data_1: &'static [&str] = &[
        "++++++++++",
        "+------+++",
        "+++-++++++",
        "+++-++++++",
        "+++-----++",
        "+++-++-+++",
        "++++++-+++",
        "++++++-+++",
        "++++++-+++",
        "++++++++++",
        "POLAND;LHASA;SPAIN;INDIA"
    ];

    const test_data_filling_1: &'static [&str] = &[
        "++++++++++",
        "+001000+++",
        "+++1++++++",
        "+++1++++++",
        "+++22232++",
        "+++1++3+++",
        "++++++3+++",
        "++++++3+++",
        "++++++3+++",
        "++++++++++"
    ];

    const test_data_2: &'static [&str] = &[
        "+-++++++++",
        "+-++++++++",
        "+-++++++++",
        "+-----++++",
        "+-+++-++++",
        "+-+++-++++",
        "+++++-++++",
        "++------++",
        "+++++-++++",
        "+++++-++++",
        "LONDON;DELHI;ICELAND;ANKARA"
    ];

    const test_data_filling_2: &'static [&str] = &[
        "+0++++++++",
        "+0++++++++",
        "+0++++++++",
        "+11112++++",
        "+0+++2++++",
        "+0+++2++++",
        "+++++2++++",
        "++333333++",
        "+++++2++++",
        "+++++2++++"
    ];

    const test_data_3: &'static [&str] = &[
        "+-++++++++",
        "+-++++++++",
        "+-------++",
        "+-++++++++",
        "+-++++++++",
        "+------+++",
        "+-+++-++++",
        "+++++-++++",
        "+++++-++++",
        "++++++++++",
        "AGRA;NORWAY;ENGLAND;GWALIOR"
    ];

    const test_data_filling_3: &'static [&str] = &[
        "+0++++++++",
        "+0++++++++",
        "+1111111++",
        "+0++++++++",
        "+0++++++++",
        "+222232+++",
        "+0+++3++++",
        "+++++3++++",
        "+++++3++++",
        "++++++++++",
    ];

    const test_data_4: &'static [&str] = &[
        "++++++-+++",
        "++------++",
        "++++++-+++",
        "++++++-+++",
        "+++------+",
        "++++++-+-+",
        "++++++-+-+",
        "++++++++-+",
        "++++++++-+",
        "++++++++-+",
        "ICELAND;MEXICO;PANAMA;ALMATY"
    ];

    const test_data_filling_4: &'static [&str] = &[
        "++++++0+++",
        "++111111++",
        "++++++0+++",
        "++++++0+++",
        "+++222223+",
        "++++++0+3+",
        "++++++0+3+",
        "++++++++3+",
        "++++++++3+",
        "++++++++3+"
    ];

    fn get_line(solver: &CrosswordSolver, index: usize) -> String {
        solver.crossword.crosswords[index].clone().into_iter().collect()
    }

    #[test]
    fn test_find_word_place() {
        let mut max = 10;
        let mut solver = create_solver(test_data_1.to_vec());
        for _ in 0..max { solver.find_word_place(); }
        assert!(!solver.find_word_place());

        let expected_chars: Vec<Vec<char>> = test_data_filling_1.iter().map(|x| x.chars().collect()).collect();
        assert_eq!(solver.crossword.crosswords, expected_chars);

        let mut solver = create_solver(test_data_2.to_vec());
        for _ in 0..max { solver.find_word_place(); }
        assert!(!solver.find_word_place());

        let expected_chars: Vec<Vec<char>> = test_data_filling_2.iter().map(|x| x.chars().collect()).collect();
        assert_eq!(solver.crossword.crosswords, expected_chars);

        let mut solver = create_solver(test_data_3.to_vec());
        for _ in 0..max { solver.find_word_place(); }
        assert!(!solver.find_word_place());

        let expected_chars: Vec<Vec<char>> = test_data_filling_3.iter().map(|x| x.chars().collect()).collect();
        assert_eq!(solver.crossword.crosswords, expected_chars);

        let mut solver = create_solver(test_data_4.to_vec());
        for _ in 0..max { solver.find_word_place(); }
        assert!(!solver.find_word_place());

        let expected_chars: Vec<Vec<char>> = test_data_filling_4.iter().map(|x| x.chars().collect()).collect();
        assert_eq!(solver.crossword.crosswords, expected_chars);
    }

    #[test]
    fn test_find_words_start_pos() {
        let mut solver = create_solver(test_data_1.to_vec());

        assert_eq!(solver.find_words_start_pos(), Some(PosWithDirection::new(Pos::new(1, 1), true)));

        let mut solver = create_solver(test_data_2.to_vec());

        let pos = solver.find_words_start_pos();
        assert_eq!(pos, Some(PosWithDirection::new(Pos::new(1, 0), false)));
        assert_eq!(solver.find_words_start_pos(), pos);
    }

    #[test]
    fn test_crossword_initial_state() {
        let solver = create_solver(test_data_1.to_vec());

        assert_eq!(solver.crossword.words, vec!["POLAND","LHASA","SPAIN","INDIA"]);
        assert_eq!(get_line(&solver,0), "++++++++++");
        assert_eq!(get_line(&solver,1), "+------+++");
        assert_eq!(solver.places.len(), 0);
        assert_eq!(solver.start_pos.x, 0);
        assert_eq!(solver.start_pos.y, 0);
        assert_eq!(solver.width, 10);
        assert_eq!(solver.height, 10);
        assert_eq!(solver.curr_pos, 0);
        assert_eq!(solver.max, 0);
    }

    #[test]
    fn test_can_start_word() {
        let nums: Vec<u8> = (0..RADIX).collect();
        let digit_chars: Vec<char> = nums.iter().map(|x| from_digit(*x as u32, RADIX as u32).unwrap()).collect();
        let all_digits = digit_chars.iter().all(|x| x.can_start_word());
        debug_assert!(all_digits);
        debug_assert!('-'.can_start_word());
        debug_assert!(!'a'.can_start_word());
        debug_assert!(!'?'.can_start_word());
    }

    #[test]
    fn test_permutor_with_predicate() {
        let mut permutor = Permutor::<i16>::with_size(1);
        assert_eq!(permutor.next_perm_with_pr(&|_, _| false), None);
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i16>::with_size(2);
        assert_eq!(permutor.next_perm_with_pr(&|x, perm| *x == 1 && perm.is_empty() || *x == 0 && !perm.is_empty()), Some(vec![1, 0]));
        assert_eq!(permutor.next_perm(), None);
    }

    #[test]
    fn test_permutor() {
        let mut permutor = Permutor::<i16>::with_size(1);
        assert_eq!(permutor.next_perm(), Some(vec![0]));
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i16>::with_size(2);
        assert_eq!(permutor.next_perm(), Some(vec![0, 1]));
        assert_eq!(permutor.next_perm(), Some(vec![1, 0]));
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i16>::with_size(3);
        assert_eq!(permutor.next_perm(), Some(vec![0, 1, 2]));
        assert_eq!(permutor.next_perm(), Some(vec![0, 2, 1]));
        assert_eq!(permutor.next_perm(), Some(vec![1, 0, 2]));
        assert_eq!(permutor.next_perm(), Some(vec![1, 2, 0]));
        assert_eq!(permutor.next_perm(), Some(vec![2, 0, 1]));
        assert_eq!(permutor.next_perm(), Some(vec![2, 1, 0]));
        assert_eq!(permutor.next_perm(), None);

        let mut permutor = Permutor::<i16>::with_size(8);
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

//    for _ in 0..2 {
//        let str = read_str();
//        println!("{}", str)
//    }
//
//    let str = read_str();
//    println!("{}", str)
}
