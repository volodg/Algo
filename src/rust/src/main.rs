use std::io;
use std::collections::HashMap;

fn is_valid(s: &str) -> bool {
    if s.is_empty() || s.len() == 1 {
        return true
    }

    let mut map = HashMap::<char, u32>::new();

    for chr in s.chars() {
        let counter = map.entry(chr).or_insert(0);
        *counter += 1
    }

    let values = map.values();

    if values.len() < 2 {
        return true
    }

    let mut map = HashMap::<u32, u32>::new();

    for val in values {
        let counter = map.entry(*val).or_insert(0);
        *counter += 1;
        if map.len() > 2 {
            return false
        }
    }

    if map.len() < 2 { return true }

    let mut key_values = map.iter();

    let (ka, a) = key_values.next().unwrap();
    let (kb, b) = key_values.next().unwrap();

    if (*ka == 1 && *a == 1) || (*kb == 1 && *b == 1) {
        return true
    }

    if *a == 1 {
        return *ka - *kb == 1
    }

    if *b == 1 {
        return *kb - *ka == 1
    }

    false
}

fn is_valid_str(s: &str) -> &'static str {
    if is_valid(s) {
        "YES"
    } else {
        "NO"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert_eq!(is_valid("aaaabbcc"), false);
        assert_eq!(is_valid("aab"), true);
        assert_eq!(is_valid("aabbcd"), false);
        assert_eq!(is_valid("aabbccddeefghi"), false);
        assert_eq!(is_valid("abcdefghhgfedecba"), true);
        assert_eq!(is_valid("ibfdgaeadiaefgbhbdghhhbgdfgeiccbiehhfcggchgghadhdhagfbahhddgghbdehidbibaeaagaeeigffcebfbaieggabcfbiiedcabfihchdfabifahcbhagccbdfifhghcadfiadeeaheeddddiecaicbgigccageicehfdhdgafaddhffadigfhhcaedcedecafeacbdacgfgfeeibgaiffdehigebhhehiaahfidibccdcdagifgaihacihadecgifihbebffebdfbchbgigeccahgihbcbcaggebaaafgfedbfgagfediddghdgbgehhhifhgcedechahidcbchebheihaadbbbiaiccededchdagfhccfdefigfibifabeiaccghcegfbcghaefifbachebaacbhbfgfddeceababbacgffbagidebeadfihaefefegbghgddbbgddeehgfbhafbccidebgehifafgbghafacgfdccgifdcbbbidfifhdaibgigebigaedeaaiadegfefbhacgddhchgcbgcaeaieiegiffchbgbebgbehbbfcebciiagacaiechdigbgbghefcahgbhfibhedaeeiffebdiabcifgccdefabccdghehfibfiifdaicfedagahhdcbhbicdgibgcedieihcichadgchgbdcdagaihebbabhibcihicadgadfcihdheefbhffiageddhgahaidfdhhdbgciiaciegchiiebfbcbhaeagccfhbfhaddagnfieihghfbaggiffbbfbecgaiiidccdceadbbdfgigibgcgchafccdchgifdeieicbaididhfcfdedbhaadedfageigfdehgcdaecaebebebfcieaecfagfdieaefdiedbcadchabhebgehiidfcgahcdhcdhgchhiiheffiifeegcfdgbdeffhgeghdfhbfbifgidcafbfcd"), true);
    }
}

fn main() {
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();

    let result = is_valid_str(input_text.trim());
    println!("{}", result)
}
