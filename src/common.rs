use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn import_file( path: String ) -> Result< Vec< String >, std::io::Error>{
  let filename = String::from( path );
  let file = File::open(filename)?;
  Ok(BufReader::new(file).lines().map(|l| l.unwrap()).collect()  )
}

pub fn string_to_int( text: Vec<String> ) -> Vec< i32 >{
  text.into_iter().map( |x| x.parse::<i32>().unwrap() ).collect()
}