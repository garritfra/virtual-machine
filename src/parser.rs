use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

/// Parses a filename and returns the instructions as bytecode
pub fn parse_file(path: &str) -> Vec<u8> {
    let mut instructions: Vec<u8> = Vec::new();
    match File::open(path) {
        Ok(f) => {
            let buffered = BufReader::new(f);

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
        }
        Err(e) => println!("File not found: {:?}", e),
    };

    instructions
}

/// Helper-Function to convert a utf8 character to into `u8`
/// Needed because `std::str::from_utf8(string.as_bytes())` returns the utf8 encoding of a character.
fn u8_from_utf8(string: &str) -> u8 {
    std::str::from_utf8(string.as_bytes())
        .unwrap()
        .parse()
        .unwrap()
}
