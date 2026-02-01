use std::collections::{HashMap, HashSet};
use std::error::Error;
use crate::formats::format::Format;
use crate::formats::json::json_format::Target::Value;
use crate::models::structured_data::StructuredData;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::DelimitedIter;

static JSON_DELIMITERS: [char; 8] = ['{', '}', '[', ']', '"', '\'', ':', ','];
static JSON_CHARS_TO_IGNORE: [char; 3] = ['\n', '\r', '\t'];

#[derive(Debug)]
pub struct JsonFormat{
    pub iter: DelimitedIter,
    pub expectation: HashSet<char>,
}

enum Target{
    Key,
    Value
}

impl JsonFormat {
    pub fn new(file:TextFile) -> Self {
        let mut json_iter = file.iter.expect("No iterator for the file.");
        json_iter.set_delimiters(&JSON_DELIMITERS);
        json_iter.set_chars_to_ignore(&JSON_CHARS_TO_IGNORE);
        JsonFormat{
            iter: json_iter,
            expectation: HashSet::from(['{', '['])
        }
    }
}


impl Format for JsonFormat {

    fn parse(&mut self, mut scope:StructuredData) -> Result<StructuredData, Box<dyn Error>> {
        let mut buf:String = String::new();
        let mut target:Target = Target::Key;

        while let Some(next) = self.expect_next() {
            match next.1 {

                Some('{') => {
                    self.set_expectation(&[' ', '}', '"', '\'']);
                    match &mut scope {
                        StructuredData::Unknown => {
                            scope = StructuredData::Object(HashMap::new());
                        },
                        StructuredData::Object(obj) => {
                            match target {
                                Target::Key => { panic!("Invalid format : Key is required.") },
                                Target::Value => {
                                    obj.insert(
                                        std::mem::take(&mut buf),
                                        self.parse( StructuredData::Object(HashMap::new()) )?
                                    );
                                }
                            }
                        },
                        StructuredData::Array(arr) => {
                            arr.push(
                                self.parse( StructuredData::Object(HashMap::new()) )?
                            );
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('{');
                        },

                        StructuredData::Number(_num) => { panic!("Invalid Format"); }
                    }
                },

                Some('}') => {
                    self.set_expectation(&[' ', ',']);
                    match &mut scope {
                        StructuredData::Object(obj) => {
                            if !next.0.trim().is_empty() {
                                obj.insert(
                                    std::mem::take(&mut buf),
                                    StructuredData::String(next.0.trim().to_string())
                                );
                            }
                            return Ok(scope);
                        },
                        StructuredData::Array(_arr) => { panic!("Invalid Format"); },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('}');
                        },

                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Number(_num) => { panic!("Invalid Format"); },
                    }
                },

                Some('[') => {
                    self.expectation.clear();
                    match &mut scope {
                        StructuredData::Unknown => {
                            scope = StructuredData::Array(Vec::new());
                        },
                        StructuredData::Object(obj) => {
                            obj.insert(
                                std::mem::take(&mut buf),
                                self.parse(StructuredData::Array(Vec::new()))?
                            );
                        },
                        StructuredData::Array(arr) => {
                            arr.push(
                                self.parse(StructuredData::Array(Vec::new()))?
                            );
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(']');
                        },

                        StructuredData::Number(num) => { panic!("Invalid Format"); }
                    }
                },

                Some(']') => {
                    self.set_expectation(&[' ', ',', ']', '}']);
                    match &mut scope {
                        StructuredData::Array(_arr) => { return Ok(scope); },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('}');
                        },

                        _ => { panic!("Invalid Format - scope:OBJECT"); }
                    }
                },

                Some('\"') | Some('\'') => {
                    match &mut scope {
                        StructuredData::Object(obj) => {
                            self.expectation.clear();
                            match target {
                                Target::Key => {
                                    buf = self
                                        .parse(StructuredData::String( String::from(next.1.unwrap())) )?
                                        .take_string()
                                        .expect("Invalid format.");
                                },
                                Target::Value => {
                                    obj.insert(
                                        std::mem::take(&mut buf),
                                        self.parse(
                                            StructuredData::String(String::from(next.1.unwrap()))
                                        )?
                                    );
                                }
                            }
                        },
                        StructuredData::Array(arr) => {
                            self.expectation.clear();
                            arr.push(
                                self.parse(
                                    StructuredData::String( String::from(next.1.unwrap()) )
                                )?
                            );
                        },
                        StructuredData::String(str) => {
                            self.set_expectation(&[' ', ',', ':', '}', ']']);
                            str.push_str(&next.0);
                            match str.chars().next() {
                                Some(opener) => { str.push(opener); }
                                _ => { panic!("어???"); }
                            }
                            return Ok(scope);
                        },

                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Number(_num) => { panic!("Invalid Format"); },
                    }
                },

                Some(':') => {
                    self.expectation.clear();
                    match &mut scope {
                        StructuredData::Object(_obj) => {
                            target = Value;
                            buf.push_str(&next.0.trim());
                            if buf.is_empty() {
                                panic!("Invalid Format : Key is empty");
                            }
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(':');
                        },

                        StructuredData::Unknown => { panic!("Invalid Format"); },
                        StructuredData::Array(_arr) => { panic!("Invalid Format"); },
                        StructuredData::Number(_num) => { panic!("Invalid Format"); }
                    }
                },

                Some(',') => {
                    match &mut scope {
                        StructuredData::Object(obj) => {
                            self.set_expectation(&[' ', '"', '\'']);
                            target = Target::Key;
                            if next.0.trim().is_empty() {
                                buf.clear();
                            }else {
                                obj.insert(
                                    std::mem::take(&mut buf),
                                    StructuredData::String(next.0.trim().to_string())
                                );
                            }
                        },
                        StructuredData::Array(arr) => {
                            self.expectation.clear();
                            if !next.0.trim().is_empty() {
                                arr.push( StructuredData::String(next.0) )
                            }
                        },
                        StructuredData::String(str) => {
                            self.expectation.clear();
                            str.push_str(&next.0);
                            str.push(',');
                        },
                        StructuredData::Number(num) => {
                            // todo 이거 필요 없는거같은데
                            *num = next.0.trim().parse()?;
                            return Ok(scope);
                        }

                        StructuredData::Unknown => { panic!("Invalid Format"); },
                    }
                },

                _ => {panic!("세상에 이런일이");}
            }
        }

        return Ok(scope);
    }

    fn export(&mut self, structure: StructuredData) -> Result<String, Box<dyn Error>> {
        todo!()
    }

}

impl StructuredData {

    fn take_string(&mut self) -> Option<String> {
        match self {
            StructuredData::String(str) => {
                return Some(std::mem::take(str));
            }
            _ => None
        }
    }

}