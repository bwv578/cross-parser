use crate::formats::xml::xml::Xml;

impl Xml {

    pub fn expect_next(&mut self) -> Option<(String, Option<char>)> {
        return self.iter.as_mut().unwrap().next();
        todo!("check expectations")
    }

}