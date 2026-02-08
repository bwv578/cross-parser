use std::string::String;
use std::collections::HashMap;
use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::BufWriter;
use std::path::Path;
use crate::formats::format::Format;
use crate::formats::json::json::{Json, Target};
use crate::formats::json::json::Target::{Key, Value};
use crate::models::structured_data::StructuredData;

impl Format for Json {

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

                        _ => { panic!("Invalid Format");todo!("shutdown with error") }
                    }
                },

                Some('}') => {
                    self.set_expectations("OBJECT_CLOSED");
                    match &mut scope {
                        StructuredData::Object(obj) => {
                            if !next.0.trim().is_empty() {
                                obj.insert(
                                    std::mem::take(&mut buf),
                                    Self::classify_json_value(&next.0)
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
                            str.push('[');
                        },

                        _ => { panic!("Invalid Format") }
                    }
                },

                Some(']') => {
                    self.set_expectations("ARRAY_CLOSED");
                    match &mut scope {
                        StructuredData::Array(arr) => {
                            if !next.0.trim().is_empty() {
                                arr.push( Self::classify_json_value(&next.0) );
                            }
                            return Ok(scope);
                        },
                        StructuredData::String(str) => {
                            str.push_str(&next.0);
                            str.push(']');
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
                                    .parse(StructuredData::String( String::new()) )?
                                    .take_string()
                                    .expect("Invalid format."); //todo!("shutdown with error")
                                self.set_expectations("KEY_READY");
                            }else {
                                obj.insert(
                                    std::mem::take(&mut buf),
                                    self.parse( StructuredData::String(String::new()) )?
                                );
                            }
                        },
                        StructuredData::Array(arr) => {
                            self.set_expectations("ANY");
                            arr.push(
                                self.parse(
                                    StructuredData::String( String::new() )
                                )?
                            );
                        },
                        StructuredData::String(str) => {
                            self.set_expectations("STRING_CLOSED");
                            str.push_str(&next.0);
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
                                    Self::classify_json_value(&next.0)
                                );
                            }
                        },
                        StructuredData::Array(arr) => {
                            self.set_expectations("ANY");
                            if !next.0.trim().is_empty() {
                                arr.push( Self::classify_json_value(&next.0) );
                            }
                        },
                        StructuredData::String(str) => {
                            self.set_expectations("ANY");
                            str.push_str(&next.0);
                            str.push(',');
                        },

                        _ => { panic!("Invalid Format"); todo!("shutdown with error")},
                    }
                },

                None => {/* Keep Iterating */},
                _ => {panic!("세상에 이런일이"); todo!("shutdown with error")}
            }
        }

        return Ok(scope);
    }

    fn export(&mut self, data: StructuredData) -> Result<String, Box<dyn Error>> {
        let path:&Path = Path::new(&self.path);

        if let Some(parent) = path.parent() {
            create_dir_all(parent).expect("Failed to create directories");
        }
        let file:File = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .expect(&"Could not open JSON file.");

        let mut writer = BufWriter::new(file);

        return self.write_data(&mut writer, data, 0);
    }

}