use std::string::String;
use std::collections::{HashMap, HashSet};
use crate::formats::json::json_error::CONTENT_STARTED;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::delimited_iter::DelimitedIter;

static JSON_DELIMITERS: [char; 8] = ['{', '}', '[', ']', '"', '\'', ':', ','];

#[derive(Debug)]
pub struct Json {
    pub iter: Option<DelimitedIter>,
    pub expectation: &'static HashSet<char>,
    pub path: String
}

#[derive(PartialEq)]
pub enum Target{
    Key,
    Value
}

impl Json {

    pub fn new(file:TextFile) -> Self {
        let mut json_iter:Option<DelimitedIter> = file.iter;
        let file_path:String = file.path;

        match &mut json_iter {
            Some(iter) => { iter.set_delimiters(&JSON_DELIMITERS); }
            None => {}
        }

        Json{
            iter: json_iter,
            expectation: CONTENT_STARTED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', '{', '['])
            }),
            path: file_path
        }
    }

}