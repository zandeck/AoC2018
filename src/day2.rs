use super::common;
use std::collections::HashMap;
use log;

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


fn words_diff_by_1(w1: &String, w2: &String) -> bool {
  w1.chars().zip(w2.chars()).filter(|(a, b)| a != b ).count() == 1
}

pub fn find_differ_by_1( w: &String, v: &[String] ) -> Option< String > {
  let mut res : Vec<String> = v.into_iter().filter( |&e| words_diff_by_1(e, &w)  ).map( |e| e.clone()).collect();
  res.pop()
}

pub fn find_identical_elements( v: &[String] ) -> (String, String) {
  let (head, tail) = v.split_at(1);
  match find_differ_by_1(head.first().unwrap(), tail) {
    None => find_identical_elements( tail ),
    Some(w) => (head.first().unwrap().to_string(), w)
  }
}

pub fn part2() -> String {
  let mut data = common::import_file("resources/input2.txt".to_string()).unwrap();
  let (w1, w2) = find_identical_elements(&data);
  let res: Vec<String> = w1.chars().zip(w2.chars()).filter(|(a, b)| a == b ).map(|(a,b)| a.to_string() ).collect();
  res.join("")
}