use std::string::String;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use crate::formats::format::Format;
use crate::formats::json::json_error::CONTENT_STARTED;
use crate::formats::json::json_format::Target::{Key, Value};
use crate::models::structured_data::StructuredData;
use crate::models::text_file::TextFile;
use crate::utils::file_utils::DelimitedIter;

static JSON_DELIMITERS: [char; 8] = ['{', '}', '[', ']', '"', '\'', ':', ','];

#[derive(Debug)]
pub struct JsonFormat{
    pub iter: DelimitedIter,
    pub expectation: &'static HashSet<char>,
}

#[derive(PartialEq)]
enum Target{
    Key,
    Value
}

impl JsonFormat {
    pub fn new(file:TextFile) -> Self {
        let mut json_iter = file.iter.expect("No iterator for the file.");
        json_iter.set_delimiters(&JSON_DELIMITERS);
        JsonFormat{
            iter: json_iter,
            expectation: CONTENT_STARTED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', '{', '['])
            })
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
                    self.set_expectations("OBJECT_OPENED");
                    match &mut scope {
                        StructuredData::Unknown => {
                            if !next.0.trim().is_empty() {
                                let hint = String::from("Found ") + &String::from(next.0);
                                self.shutdown_with_error("Invalid format", &hint);
                            }
                            scope = StructuredData::Object(HashMap::new());
                        },
                        StructuredData::Object(obj) => {
                            if target == Key {
                                self.shutdown_with_error("Invalid format", "Key is required.");
                            }
                            obj.insert(
                                std::mem::take(&mut buf),
                                self.parse( StructuredData::Object(HashMap::new()) )?
                            );
                        },
                        StructuredData::Array(arr) => {
                            arr.push( self.parse( StructuredData::Object(HashMap::new()) )? );
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('{');
                        },

                        StructuredData::Number(_num) => {
                            panic!("Invalid Format");
                            todo!("shutdown with error")
                        }
                    }
                },

                Some('}') => {
                    self.set_expectations("OBJECT_CLOSED");
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
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('}');
                        },

                        _ => { panic!("Invalid Format"); todo!("shutdown with error") }
                    }
                },

                Some('[') => {
                    self.set_expectations("ANY");
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

                        StructuredData::Number(num) => { panic!("Invalid Format"); todo!("shutdown with error")}
                    }
                },

                Some(']') => {
                    self.set_expectations("ARRAY_CLOSED");
                    match &mut scope {
                        StructuredData::Array(_arr) => { return Ok(scope); },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push('}');
                        },

                        _ => {self.shutdown_with_error("Invalid format", "Found ].");}
                    }
                },

                Some('\"') | Some('\'') => {
                    match &mut scope {
                        StructuredData::Object(obj) => {
                            self.set_expectations("ANY");
                            if target == Key {
                                buf = self
                                    .parse(StructuredData::String( String::from(next.1.unwrap())) )?
                                    .take_string()
                                    .expect("Invalid format."); //todo!("shutdown with error")
                                self.set_expectations("KEY_READY");
                            }else {
                                obj.insert(
                                    std::mem::take(&mut buf),
                                    self.parse( StructuredData::String(String::from(next.1.unwrap())) )?
                                );
                            }
                        },
                        StructuredData::Array(arr) => {
                            self.set_expectations("ANY");
                            arr.push(
                                self.parse(
                                    StructuredData::String( String::from(next.1.unwrap()) )
                                )?
                            );
                        },
                        StructuredData::String(str) => {
                            self.set_expectations("STRING_CLOSED");
                            str.push_str(&next.0);
                            match str.chars().next() {
                                Some(opener) => { str.push(opener); }
                                _ => { panic!("어???"); }
                            }
                            return Ok(scope);
                        },

                        _ => { panic!("Invalid Format"); todo!("shutdown with error") }
                    }
                },

                Some(':') => {
                    self.set_expectations("ANY");
                    match &mut scope {
                        StructuredData::Object(_obj) => {
                            target = Value;
                            buf.push_str(&next.0.trim());
                            if buf.is_empty() {
                                self.shutdown_with_error("Invalid format", "Key is required.");
                            }
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(':');
                        },

                        _ => { panic!("Invalid Format"); todo!("shutdown with error")}
                    }
                },

                Some(',') => {
                    match &mut scope {
                        StructuredData::Object(obj) => {
                            self.set_expectations("KEY_VALUE_PAIRED");
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
                            self.set_expectations("ANY");
                            if !next.0.trim().is_empty() {
                                arr.push( StructuredData::String(next.0) )
                            }
                        },
                        StructuredData::String(str) => {
                            self.set_expectations("ANY");
                            str.push_str(&next.0);
                            str.push(',');
                        },
                        StructuredData::Number(num) => {
                            // todo 이거 필요 없는거같은데
                            *num = next.0.trim().parse()?;
                            return Ok(scope);
                        }

                        StructuredData::Unknown => { panic!("Invalid Format"); todo!("shutdown with error")},
                    }
                },

                None => {/* Keep Iterating */},
                _ => {panic!("세상에 이런일이"); todo!("shutdown with error")}
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
            StructuredData::String(str) => Some(std::mem::take(str)),
            _ => None
        }
    }

}