use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

pub fn import_file(path: String) -> Result<Vec<String>, std::io::Error> {
  let filename = String::from(path);
  let file = File::open(filename)?;
  Ok(BufReader::new(file).lines().map(|l| l.unwrap()).collect())
}

pub fn import_file_to_u8(path: String) -> Result<String, std::io::Error> {
  let filename = String::from(path);
  let file = File::open(filename)?;
  let mut data: String = String::new();
  BufReader::new(file).read_to_string(&mut data);

  Ok(data)
}

pub fn string_to_int(text: Vec<String>) -> Vec<i32> {
  text
    .into_iter()
    .map(|x| x.parse::<i32>().unwrap())
    .collect()
}
