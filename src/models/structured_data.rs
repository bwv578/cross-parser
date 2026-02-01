use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
pub enum StructuredData {
    Object(HashMap<String, StructuredData>),
    Array(Vec<StructuredData>),
    String(String),
    Number(f64),
    Unknown
}