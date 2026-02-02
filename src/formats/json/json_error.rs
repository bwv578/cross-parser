use std::collections::HashSet;
use std::sync::OnceLock;
use crate::formats::json::json_format::JsonFormat;


pub static CONTENT_STARTED:OnceLock<HashSet<char>> = OnceLock::new();
static OBJECT_OPENED:OnceLock<HashSet<char>> = OnceLock::new();
static OBJECT_CLOSED: OnceLock<HashSet<char>> = OnceLock::new();
static ARRAY_CLOSED: OnceLock<HashSet<char>> = OnceLock::new();
static STRING_CLOSED: OnceLock<HashSet<char>> = OnceLock::new();
static KEY_READY: OnceLock<HashSet<char>> = OnceLock::new();
static KEY_VALUE_PAIRED: OnceLock<HashSet<char>> = OnceLock::new();
static ANY: OnceLock<HashSet<char>> = OnceLock::new();
static JSON_CHARS_TO_IGNORE: OnceLock<HashSet<char>> = OnceLock::new();

impl JsonFormat {

    pub fn set_expectations(&mut self, situation:&str) {
        self.expectation = match situation {
            "CONTENT_STARTED" => CONTENT_STARTED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', '{', '['])
            }),
            "OBJECT_OPENED" => OBJECT_OPENED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', '}', '"', '\''])
            }),
            "OBJECT_CLOSED" => OBJECT_CLOSED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', ',', ']'])
            }),
            "ARRAY_CLOSED" => ARRAY_CLOSED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', ',', ':', '}', ']'])
            }),
            "STRING_CLOSED" => STRING_CLOSED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', ',', ':', '}', ']'])
            }),
            "KEY_READY" => KEY_VALUE_PAIRED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', ':'])
            }),
            "KEY_VALUE_PAIRED" => KEY_VALUE_PAIRED.get_or_init(|| {
                HashSet::from([' ', '\n', '\r', '\t', '"', '\''])
            }),
            "ANY" => ANY.get_or_init(|| {
                HashSet::from([])
            }),
            _ => panic!("Invalid situation.")
        }
    }

    pub fn expect_next(&mut self) -> Option<(String, Option<char>)> {
        let ignores: &HashSet<char> = JSON_CHARS_TO_IGNORE.get_or_init(|| {
            HashSet::from(['\n', '\r', '\t', ' '])
        });

        match self.iter.next() {
            Some(next) => {
                let v:char = next.0
                    .chars()
                    .next()
                    .unwrap_or( next.1.unwrap() );

                if self.expectation.is_empty() || self.expectation.contains(&v) {
                    return Some(next);
                }else {
                    let lines_jumped = next.0.
                        chars().filter(|&c| c == '\n').count();

                    if next.0.len()>0 {
                        println!("Error: Invalid JSON format - line:{}, col:{}",
                                 self.iter.pos.0-lines_jumped, self.iter.pos.1+lines_jumped-next.0.len());
                    }else {
                        println!("Error: Invalid JSON format - line:{}, col:{}",
                                 self.iter.pos.0-lines_jumped, self.iter.pos.1);
                    }

                    print!("Hint: Expected ");
                    for x in self.expectation {
                        if !ignores.contains(x){ print!("{} ", x); }
                    }
                    println!("but  found {}", v);
                    std::process::exit(1);
                }
            }

            None => {
                if self.expectation.is_empty() { return None; }
                println!("Error: Invalid JSON format - line:{}, col:{}",
                         self.iter.pos.0, self.iter.pos.1);

                print!("Hint: Expected ");
                for x in self.expectation {
                    if !ignores.contains(x){ print!("{} ", x); }
                }

                println!("but found nothing.");
                std::process::exit(1);
            }
        }
    }
}
