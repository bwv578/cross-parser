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
