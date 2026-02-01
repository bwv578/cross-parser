use std::io::Error;
use crate::utils::string_utils::mirror;
use crate::utils::file_utils::{DelimitedIter};

#[derive(Debug)]
pub struct TextFile {
    pub name:String,
    pub format:String,
    pub iter: Option<DelimitedIter>,
}

impl TextFile {

    pub fn new(file_name:&String) -> Self {
        let mut inferred_format:String = String::from("");

        let vectorized:Vec<char> = file_name.chars().collect();
        let mut i:usize = vectorized.len();

        while i>0 {
            let c:char = vectorized[i-1];
            if c == '.' {break;}

            inferred_format.push(c);
            i-=1;
        }

        return Self {
            name: file_name.clone(),
            format: mirror(&inferred_format),
            iter: None
        }
    }

    pub fn set_name(self:&mut Self, new_name:&String) -> &mut Self {
        self.name = new_name.to_string();
        return self;
    }

    pub fn set_format(self:&mut Self, new_format:&String) -> &mut Self {
        self.format = new_format.to_string();
        return self;
    }

    pub fn append_name(self:&mut Self, suffix:&String) -> &mut Self {
        self.name.push_str(suffix);
        return self;
    }

    pub fn shell_copy(self:&mut Self) -> Self {
        return Self {
            name: self.name.clone(),
            format: self.format.clone(),
            iter: None
        }
    }

    pub fn impl_as_reader(self:&mut Self) -> &mut Self {
        let iter_result:Result<DelimitedIter, Error> = DelimitedIter::new(&self.name);
        match iter_result {
            Ok(mut iter) => {
                iter.set_chars_to_ignore(&[' ', '\r', '\n', '\t']);
                self.iter = Some(iter);
            }
            Err(_) => {panic!("Invalid file path: {}", self.name)}
        }
        return self;
    }

    pub fn impl_as_writer(){}

}
