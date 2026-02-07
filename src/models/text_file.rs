use std::io::Error;
use crate::utils::string_utils::mirror;
use crate::utils::file_utils::delimited_iter::{DelimitedIter};

#[derive(Debug)]
pub struct TextFile {
    pub path:String,
    pub format:String,
    pub iter: Option<DelimitedIter>,
}

impl TextFile {

    pub fn new(file_path:&String) -> Self {
        let mut inferred_format:String = String::from("");

        let vectorized:Vec<char> = file_path.chars().collect();
        let mut i:usize = vectorized.len();

        while i>0 {
            let c:char = vectorized[i-1];
            if c == '.' {break;}

            inferred_format.push(c);
            i-=1;
        }

        return Self {
            path: file_path.clone(),
            format: mirror(&inferred_format),
            iter: None
        }
    }

    pub fn set_path(self:&mut Self, new_path:&String) -> &mut Self {
        self.path = new_path.to_string();
        return self;
    }

    pub fn set_format(self:&mut Self, new_format:&String) -> &mut Self {
        self.format = new_format.to_string();
        return self;
    }

    pub fn append_path(self:&mut Self, suffix:&String) -> &mut Self {
        self.path.push_str(suffix);
        return self;
    }

    pub fn shell_copy(self:&mut Self) -> Self {
        return Self {
            path: self.path.clone(),
            format: self.format.clone(),
            iter: None
        }
    }

    pub fn impl_as_reader(self:&mut Self) -> &mut Self {
        let iter_result:Result<DelimitedIter, Error> = DelimitedIter::new(&self.path);
        match iter_result {
            Ok(mut iter) => { self.iter = Some(iter); }
            Err(_) => {panic!("Invalid file path: {}", self.path)}
        }
        return self;
    }

    pub fn impl_as_writer(){}

}
