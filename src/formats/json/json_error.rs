use crate::formats::json::json_format::JsonFormat;

impl JsonFormat {

    pub fn set_expected(&mut self, chars:&[char]) {
        self.expected.extend(chars.iter());
    }

    pub fn expect(&mut self) -> Option<(String, Option<char>)> {
        match self.iter.next() {
            Some(next) => {
                let v:char = next.0
                    .chars()
                    .next()
                    .unwrap_or( next.1.unwrap() );

                if self.expected.is_empty() || self.expected.contains(&v) {
                    self.expected.clear();
                    return Some(next);
                }else {
                    println!("crossp: error occurred while parsing text.");
                    println!(" !! Invalid JSON format - line:{}, col:{}", self.iter.pos.1, self.iter.pos.0);
                    println!(" !! Expected values : {:?}", self.expected);
                    println!(" !! Found : {}", v);
                    std::process::exit(1);
                }
            }
            None => {
                if self.expected.is_empty() { return None; }
                println!("crossp: error occurred while parsing text.");
                println!(" !! Invalid JSON format - line:{}, col:{}", self.iter.pos.1, self.iter.pos.0);
                println!(" !! Expected values : {:?}", self.expected);
                println!(" !! Found nothing.");
                std::process::exit(1);
            }
        }
    }
}
