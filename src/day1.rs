use super::common;
use std::collections::HashSet;

fn get_data() -> Vec< i32 > {
  let data = common::import_file(String::from("resources/input1.txt")).unwrap();
  common::string_to_int(data)
}

pub fn problem1() -> i32 {
  let data = get_data();
  data.into_iter().sum()
}

pub fn problem2() -> i32 {
  let data = get_data();
  let mut m = HashSet::new();
  m.insert(0);
  let mut current_freq: i32 = data[0];
  let mut current_index: usize = 1;
  
  while !m.contains(&current_freq) {
    m.insert(current_freq);
    current_freq += data[current_index];
    current_index = if ( current_index + 1 >= data.len()) {
      0
    } else {
      current_index + 1
    }
  }
  
  current_freq
}