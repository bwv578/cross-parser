use std::error::Error;

use crate::models::structured_data::StructuredData;

pub trait Format{
    fn parse (&mut self, scope:StructuredData) -> Result<StructuredData, Box<dyn Error>>;
    fn export (&mut self, structure:StructuredData) -> Result<String, Box<dyn Error>>;
}