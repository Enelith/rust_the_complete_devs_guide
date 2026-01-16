// 'std' = 'Standard Library', 'fs' = 'File System'
use std::fs;

fn main() {
    version_working_fine();
    println!("\n-----------------------------\n");

    version_error_lifetime();
    println!("\n-----------------------------\n");
}

fn version_working_fine() {
    println!("VERSION 1:");

    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);
    // Will return: "OK( 'INFO 14:30:25 Application started.\nWARN.... Network connectivity restored.',)"
    // which is very similar to the previous Option enum (Some, None)

    match text {
        Ok(text_that_was_read) => {
            println!("Text length: {}", text_that_was_read.len());
            // Either convert text_that_was_read into a &String (which is fine bcs Rust will convert it to a &str as expected by the method)
            let errors_string_ref = extract_errors(&text_that_was_read);
            // Or convert text_that_was_read into a &str using .as_str()
            let errors_string_slice = extract_errors(text_that_was_read.as_str());

            println!("[Ref] Found {} errors: {:#?}", errors_string_ref.len(), errors_string_ref);
            println!("[Slice] Found {} errors: {:#?}", errors_string_slice.len(), errors_string_slice);

            let errors_evolved = extract_errors_2(text_that_was_read.as_str());
            println!("[Evolved] Found {} errors: {:#?}", errors_evolved.len(), errors_evolved);
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }
}

fn version_error_lifetime() {
    println!("VERSION 2:");

    let text = fs::read_to_string("logs.txt");
    println!("{:#?}", text);

    let mut errors = Vec::new();

    match text {
        Ok(text_that_was_read) => {
            println!("Text length: {}", text_that_was_read.len());

            // text_that_was_read will show an error as it is: `text_that_was_read` does not live long enough [E0597]
            errors = extract_errors(text_that_was_read.as_str());

        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }

    println!("Found {} errors: {:#?}", errors.len(), errors);
}

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut result = Vec::new();

    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line);
        }
    }

    result
}

fn extract_errors_2(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| line.starts_with("ERROR"))
        .collect()
}