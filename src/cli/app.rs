use std::collections::HashMap;
use crate::models::text_file::TextFile;
use crate::formats::format::Format;
use crate::formats::json::json_format::JsonFormat;
use crate::models::structured_data::StructuredData;

pub fn parse_options(args:&Vec<String>) -> HashMap<String, String>{
    let mut options:HashMap<String, String> = HashMap::new();
    let mut k:String = String::new();

    for arg in args.iter().skip(0) {
        match arg.chars().take(2).collect::<String>().as_str() {
            "--" => {
                k = arg.chars().skip(2).collect();
            }
            _ => {
                options.insert(k.clone(), arg.clone());
            }
        }
    }

    return options;
}


pub fn execute(options:HashMap<String, String>) {

    let mut source:TextFile = match options.get(&String::from("from")) {
        Some(file_path) => TextFile::new(file_path),
        _ => {panic!("Source file is required.\nex) --from <path>");}
    };
    source.impl_as_reader();

    let mut target:TextFile = match options.get(&String::from("out")) {
        Some(file_path) => TextFile::new(file_path),
        _ => source.shell_copy()
    };

    match options.get(&String::from("as")) {
        Some(format) => {source.set_format(format);}
        _ => {}
    }

    match options.get(&String::from("to")) {
        Some(format) => {
            target
                .append_name(&String::from("."))
                .append_name(format)
                .set_format(format);
        }
        _ => {}
    }

    let mut formatter = get_formatter(source).expect("No formatter available.");
    let result:StructuredData = (*formatter).parse(StructuredData::Unknown);
    
    println!("structured :  {:#?}", result);
}


pub fn get_formatter(file:TextFile) -> Option<Box<dyn Format>> {
    match file.format.as_str() {
        "json" => Some(Box::new(JsonFormat::new(file))),
        _ => None,
    }
}