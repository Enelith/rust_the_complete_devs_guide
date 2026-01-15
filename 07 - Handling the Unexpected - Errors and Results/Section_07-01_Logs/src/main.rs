// 'std' = 'Standard Library', 'fs' = 'File System'
use std::fs;

fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);
    // Will return: "OK( 'INFO 14:30:25 Application started.\nWARN.... Network connectivity restored.',)"
    // which is very similar to the previous Option enum (Some, None)

    match text {
        Ok(text_that_was_read) => {
            println!("Text length: {}", text_that_was_read.len());
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }
}
