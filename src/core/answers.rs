
use std::fs::File;
use std::io::Read;
use super::response::Response;

pub fn answer_file(response: &mut Response, status_code: u16, file_path: &str) {
  let mut file = File::open(file_path)
    .expect(&["could not open ", file_path].concat());

  let mut file_content = String::new();
  file.read_to_string(&mut file_content).unwrap();
  
  response.set_response(
    &format!("HTTP/1.1 {} OK\r\n\r\n{}", status_code, file_content)
  )
}


pub fn answer_text(response: &mut Response, status_code: u16, text: &str) {
  response.set_response(
    &format!("HTTP/1.1 {} OK\r\n\r\n{}", status_code, text)
  )
}