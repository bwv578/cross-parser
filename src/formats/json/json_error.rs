use crate::formats::json::json_format::JsonFormat;

impl JsonFormat {

    pub fn set_expectation(&mut self, chars:&[char]) {
        self.expectation.clear();
        self.expectation.extend(chars.iter());
    }

    pub fn expect_next(&mut self) -> Option<(String, Option<char>)> {
        match self.iter.next() {
            Some(next) => {
                let v:char = next.0
                    .chars()
                    .next()
                    .unwrap_or( next.1.unwrap() );

                if self.expectation.is_empty() || self.expectation.contains(&v) {
                    //self.expectation.clear();
                    return Some(next);
                }else {
                    println!("crossp: error occurred while parsing text.");
                    println!(" !! Invalid JSON format - line:{}, col:{}~{}",
                             self.iter.pos.0, self.iter.pos.1-next.0.len(), self.iter.pos.1);
                    println!(" !! Expected : {:?}", self.expectation);
                    println!(" !! Found : {}", v);
                    std::process::exit(1);
                }
            }
            None => {
                if self.expectation.is_empty() { return None; }
                println!("crossp: error occurred while parsing text.");
                println!(" !! Invalid JSON format - line:{}, col:{}", self.iter.pos.0, self.iter.pos.1);
                println!(" !! Expected : {:?}", self.expectation);
                println!(" !! Found nothing.");
                std::process::exit(1);
            }
        }
    }
}
