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
            "KEY_READY" => KEY_READY.get_or_init(|| {
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
        let iter = self.iter.as_mut().unwrap();

        let ignores: &HashSet<char> = JSON_CHARS_TO_IGNORE.get_or_init(|| {
            HashSet::from(['\n', '\r', '\t', ' '])
        });

        match iter.next() {
            Some(next) => {
                let v:char;
                if next.0.chars().next() == None {
                    if next.1 == None {return self.expect_next();}
                    v = next.1.unwrap();
                }else {
                    v = next.0.chars().next().unwrap();
                }

                let lines_jumped = next.0.
                    chars().filter(|&c| c == '\n').count();

                let line:usize =  iter.pos.0-lines_jumped;
                let mut col = iter.pos.1 - next.0.chars()
                    .rev()
                    .take_while(|&c| c != '\n')
                    .count();

                if col==0 {col+=1;}

                if self.expectation.is_empty() || self.expectation.contains(&v) {
                    return Some(next);
                }else {
                    println!("Error: Invalid JSON format: line:{}, col:{}", line, col);
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
                println!("Error: Invalid JSON format: line:{}, col:{}",
                         iter.pos.0, iter.pos.1);
                print!("Hint: Expected ");
                for x in self.expectation {
                    if !ignores.contains(x){ print!("{} ", x); }
                }
                println!("but found nothing.");
                std::process::exit(1);
            }
        }
    }

    pub fn shutdown_with_error(&mut self, message: &str, hint: &str) {
        let iter = self.iter.as_ref().unwrap();

        let col = if iter.pos.1 == 0 {
            iter.pos.0 + 1
        }else {
            iter.pos.1
        };
        println!("Error: {} : line:{}, col:{}", message, iter.pos.0, col);
        println!("Hint: {}", hint);
        std::process::exit(1);
    }

}
