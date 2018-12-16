use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

pub fn parse_file(path: String) -> Result<Vec<u8>, std::io::Error> {
  let mut instructions: Vec<u8> = Vec::new();
  let input = File::open(path)?;
  let buffered = BufReader::new(input);

  for line in buffered.lines() {
    for word in line.unwrap().split_whitespace() {
      match word.as_ref() {
        "nop" => instructions.push(0x00),
        "push" => instructions.push(0x01),
        "print" => instructions.push(0x02),
        "add" => instructions.push(0x03),
        "halt" => instructions.push(0xff),
        x => instructions.push(u8_from_utf8(x)),
      }
    }
  }

  Ok(instructions)
}

fn u8_from_utf8(string: &str) -> u8 {
  std::str::from_utf8(string.as_bytes())
    .unwrap()
    .parse()
    .unwrap()
}
