use super::common;
use std::collections::HashMap;

pub fn count_letter_in_word(word: &String) -> HashMap<char, u32> {
    let mut count_letter = HashMap::new();
    for c in word.chars() {
        if count_letter.contains_key(&c) {
            let counter = count_letter.get(&c).unwrap();
            count_letter.insert(c, counter + 1);
        } else {
            count_letter.insert(c, 1);
        }
    }

    count_letter
}

pub fn contains_exactly(n: u32, word: &String) -> bool {
    count_letter_in_word(&word).values().any(|e| *e == n)
}

pub fn return_hash(l: Vec<String>) -> u32 {
    let a = l.iter().filter(|e| contains_exactly(2, e)).count() as u32;
    let b = l.iter().filter(|e| contains_exactly(3, e)).count() as u32;
    a * b
}

pub fn part1() -> u32 {
    let data = common::import_file("resources/input2.txt".to_string()).unwrap();
    return_hash(data)
}
