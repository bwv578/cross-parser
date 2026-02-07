use crate::models::structured_data::StructuredData;

pub fn mirror(original:&String) -> String {

    let vectorized:Vec<char> = original.chars().collect(); 
    let mut mirrored:String = String::from("");

    let mut i:usize = original.len();
    while i>0 {
        mirrored.push(vectorized[i-1]);
        i-=1;
    }

    return mirrored;
}

pub fn classify_value(value:&str) -> StructuredData {
    let trimmed = value.trim();

    return match trimmed.parse::<f64>() {
        Ok(_num) => StructuredData::Number(trimmed.to_string()),
        Err(_) => {
            if trimmed == "true" || trimmed == "false" {
                StructuredData::Boolean(trimmed.to_string())
            } else {
                StructuredData::String(value.to_string())
            }
        }
    }
}