pub mod cli;
pub mod formats;
pub mod utils;
pub mod models;


#[cfg(test)]
pub mod test{
    use crate::cli::app::{execute, parse_options};

    #[test]
    fn test_execute(){

        // args
        let env:Vec<String> = vec![
            String::from("./parser"),
            String::from("--from"),
            String::from("example/test_script-1.json"),
            String::from("--to"),
            String::from("xml"),
        ];

        execute(parse_options(&env));
    }
}