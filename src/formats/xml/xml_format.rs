use std::io::Error;
use crate::formats::format::Format;
use crate::models::intermediate_structure::IntermediateStructure;
use crate::models::text_file::TextFile;

pub struct XmlFormat{
    file_to_read:Option<Box<TextFile>>,
    intermediate_struct: Option<Box<IntermediateStructure>>
}

impl Format for XmlFormat {
    fn construct(&mut self, file: TextFile) -> IntermediateStructure {
        todo!()
    }
    fn export(&mut self, structure: IntermediateStructure) -> Result<String, Error> {
        todo!()
    }
}