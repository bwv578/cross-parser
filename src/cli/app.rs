use std::collections::HashMap;
use crate::models::text_file::TextFile;
use crate::formats::format::Format;
use crate::formats::json::json::Json;
use crate::formats::xml::xml::Xml;
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
                .append_path(&String::from("."))
                .append_path(format)
                .set_format(format);
        }
        _ => {}
    }

    // File => Structure
    let mut formatter = get_formatter(source).expect("No formatter available.");
    let format_result = match (*formatter).parse(StructuredData::Unknown) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Parsing error: {}", e);
            std::process::exit(1);
        }
    };

    // Structure => File
    let mut exporter = get_formatter(target).expect("No exporter available.");
    let export_result = match (*exporter).export(format_result) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("File write error: {}", e);
            std::process::exit(1);
        }
    };
    println!("export process : {:#?}", export_result);

}


pub fn get_formatter(file:TextFile) -> Option<Box<dyn Format>> {
    match file.format.as_str() {
        "json" => Some(Box::new(Json::new(file))),
        "box" => Some(Box::new(Xml::new(file))),
        _ => None,
    }
}