use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufReader, Error, Read, BufWriter};

#[derive(Debug)]
pub struct DelimitedIter {
    out: (String, Option<char>),
    delimiters: HashSet<char>,
    chars_to_ignore: HashSet<char>,
    reader: Option<BufReader<File>>
}

impl DelimitedIter {

    pub fn new(file_path:&String) -> Result<DelimitedIter, Error> {
        let file_to_read:File = File::open(file_path)?;
        return Ok(
            Self {
                out: (String::new(), None),
                delimiters: HashSet::new(),
                chars_to_ignore: HashSet::new(),
                reader: Some(BufReader::new(file_to_read))
            }
        )
    }

    pub fn set_delimiters(&mut self, delimiters: &[char]) -> &mut Self {
        self.delimiters.clear();
        for delimiter in delimiters {
            self.delimiters.insert(*delimiter);
        }
        return self;
    }

    pub fn set_chars_to_ignore(&mut self, chars: &[char]) -> &mut Self {
        self.chars_to_ignore.clear();
        for ig_char in chars {
            self.chars_to_ignore.insert(*ig_char);
        }
        return self;
    }

    pub fn next(&mut self) -> Option<(String, Option<char>)> {
        self.out = (String::new(), None);
        let mut buff = [0u8; 1];
        let reader = self.reader.as_mut().expect("No file reader");

        loop {
            match reader.read(&mut buff) {
                Ok(0) => {
                    if self.out.0.is_empty() {
                        return None;
                    }else{
                        return Some(std::mem::take(&mut self.out));
                    }
                }
                Ok(_) => {
                    let c:char = buff[0] as char;
                    if self.delimiters.contains(&c) {
                        self.out.1 = Some(c);
                        return Some(std::mem::take(&mut self.out));
                    }else if !self.chars_to_ignore.contains(&c) {
                        self.out.0.push(c);
                    }
                }
                Err(e) => {panic!("File Error: {}", e);}
            }
        }
    }

}