use std::collections::HashSet;
use std::io::Error;
use crate::formats::format::Format;
use crate::models::structured_data::StructuredData;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::delimited_iter::DelimitedIter;

#[derive(Debug)]
pub struct XmlFormat{
    pub iter: Option<DelimitedIter>,
    pub expectation: &'static HashSet<char>,
    path: String
}

impl Format for XmlFormat {
    fn parse(&mut self, scope: StructuredData) -> Result<StructuredData, Box<dyn std::error::Error>> {

        let mut buf:String = String::new();
        //let mut target: crate::formats::json::json_format::Target = crate::formats::json::json_format::Target::Key;

        /*while let Some(next) = self.expect_next() {
            match next.1 {


            }
        }*/

        /*match scope {
            StructuredData::Unknown => {},
            StructuredData::Object(obj) => {},
            StructuredData::Array(arr) => {},
            StructuredData::String(str) => {},
            StructuredData::Number(num) => {},
            StructuredData::Boolean(bool) => {},
        }*/

        return Ok(scope);
    }

    fn export (&mut self, scope: StructuredData) -> Result<String, Box<dyn std::error::Error>> {
        todo!()

    }
}