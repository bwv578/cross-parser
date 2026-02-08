use crate::formats::format::Format;
use crate::formats::xml::xml::*;
use crate::models::structured_data::StructuredData;

impl Format for Xml {
    fn parse(&mut self, mut scope: StructuredData) -> Result<StructuredData, Box<dyn std::error::Error>> {

        let mut buf:String = String::new();
        let mut target:Target = Target::TagName;

        while let Some(next) = self.expect_next() {
            match next.1 {

                Some('<') => {
                    match &mut scope {
                        StructuredData::Unknown => {
                            target = Target::TagName;
                            buf.clear();
                        },
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('>') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('/') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('?') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('"') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('\'') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('=') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('!') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some('-') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                Some(' ') => {
                    match &mut scope {
                        StructuredData::Unknown => {},
                        StructuredData::Object(obj) => {},
                        StructuredData::Array(arr) => {},
                        StructuredData::String(str) => {},
                        StructuredData::Number(num) => {},
                        StructuredData::Boolean(boo) => {}
                    }
                },

                None => {},
                _ => {}
            }
        }

        return Ok(scope);
    }

    fn export (&mut self, scope: StructuredData) -> Result<String, Box<dyn std::error::Error>> {
        todo!()

    }
}