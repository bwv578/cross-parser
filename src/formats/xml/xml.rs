use std::collections::HashSet;
use crate::formats::json::json_error::CONTENT_STARTED;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::delimited_iter::DelimitedIter;

static XML_DELIMITERS: [char; 10] = ['<', '/', '>', '?', '"', '!', '-', '\'', '=', ' '];

#[derive(Debug)]
pub struct Xml {
    pub iter: Option<DelimitedIter>,
    pub expectation: &'static HashSet<char>,
    path: String
}

pub enum Target {
    TagName, TagOptions, Value
}

impl Xml {
    pub fn new(file:TextFile) -> Self {
        let mut xml_iter:Option<DelimitedIter> = file.iter;
        let file_path:String = file.path;

        match &mut xml_iter {
            Some(iter) => { iter.set_delimiters(&XML_DELIMITERS); }
            None => {}
        }

        Xml {
            iter: xml_iter,
            expectation: CONTENT_STARTED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', '{', '['])
            }),
            path: file_path
        }
    }

}