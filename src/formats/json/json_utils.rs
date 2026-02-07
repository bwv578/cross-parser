use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use crate::utils::string_utils::classify_value;
use crate::formats::json::json_format::JsonFormat;
use crate::models::structured_data::StructuredData;

impl JsonFormat {

    pub fn write_data(&mut self, writer:&mut BufWriter<File>, data:StructuredData, depth:usize) -> Result<String, Box<dyn Error>> {
        match data {
            StructuredData::Unknown => {
                return Err(Box::new(std::fmt::Error))
            },

            StructuredData::Object(obj) => {
                let len = obj.len();

                writeln!(writer, "{{")?;
                for (i, (key, value)) in obj.into_iter().enumerate() {
                    Self::indent(writer, depth+1)?;
                    write!(writer, "\"{}\" : ", key)?;

                    self.write_data(writer, value, depth+1)?;
                    if i!=len-1 { write!(writer, ",\n")?; }
                    else { write!(writer, "\n")?; }
                }
                Self::indent(writer, depth)?; write!(writer, "}}")?;
            },

            StructuredData::Array(arr) => {
                let len = arr.len();

                writeln!(writer, "[")?;
                for (i, elem) in arr.into_iter().enumerate() {
                    Self::indent(writer, depth+1)?;

                    self.write_data(writer, elem, depth+1)?;
                    if i!=len-1 { writeln!(writer, ",")?; }
                    else { write!(writer, "\n")?; }
                }
                Self::indent(writer, depth)?; write!(writer, "]")?;
            },

            StructuredData::String(str) => { write!(writer, "\"{}\"", str)?; },
            StructuredData::Number(num) => { write!(writer, "{}", num)?; },
            StructuredData::Boolean(boo) => { write!(writer, "{}", boo)?; }
        }

        if depth==0 { writeln!(writer)?; }
        return Ok(String::from("DONE"));
    }

    pub fn indent(writer:&mut BufWriter<File>, depth:usize) -> Result<bool, Box<dyn Error>> {
        for _ in 0..depth { write!(writer, "\t")?; }
        return Ok(true);
    }

    pub fn classify_json_value(value:&str) -> StructuredData {
        return match classify_value(value) {
            StructuredData::Number(num) => { StructuredData::Number(num) },
            StructuredData::Boolean(bool) => { StructuredData::Boolean(bool) },
            _ => { panic!("Invalid Format.") }
        }
    }

}