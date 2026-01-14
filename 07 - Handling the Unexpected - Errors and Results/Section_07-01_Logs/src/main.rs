// 'std' = 'Standard Library', 'fs' = 'File System'
use std::fs;

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);
    // Will return: "OK( 'INFO 14:30:25 Application started.\nWARN.... Network connectivity restored.',)"
    // which is very similar to the previous Option enum (Some, None)
}
