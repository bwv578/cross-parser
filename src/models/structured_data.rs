use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub enum StructuredData {
    Object(HashMap<String, StructuredData>),
    Array(Vec<StructuredData>),
    String(String),
    Number(String),
    Boolean(String),
    Unknown
}

impl StructuredData {
    pub fn take_string(&mut self) -> Option<String> {
        match self {
            StructuredData::String(str) => Some(std::mem::take(str)),
            _ => None
        }
    }

}