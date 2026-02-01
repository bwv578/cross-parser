use std::collections::HashMap;
use std::io::Error;

use crate::models::structured_data::StructuredData;

pub trait Format{
    fn parse (&mut self, scope:StructuredData) -> StructuredData;
    fn export (&mut self, structure:StructuredData) -> Result<String, Error>;
}