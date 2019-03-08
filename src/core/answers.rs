
use std::fs::File;
use std::io::Read;

pub fn answer_file(file_path: &str) -> String {
  let mut file = File::open(file_path)
    .expect(&["could not open ", file_path].concat());

  let mut file_content = String::new();
  file.read_to_string(&mut file_content).unwrap();

  file_content
}