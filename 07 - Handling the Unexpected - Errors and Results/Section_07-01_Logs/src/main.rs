// 'std' = 'Standard Library', 'fs' = 'File System'
use std::fs;

fn main() {
    version_working_fine();
    println!("\n-----------------------------\n");

    version_error_lifetime();
    println!("\n-----------------------------\n");

    writing_data_to_file_process();
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

            println!(
                "[Ref] Found {} errors: {:#?}",
                errors_string_ref.len(),
                errors_string_ref
            );
            println!(
                "[Slice] Found {} errors: {:#?}",
                errors_string_slice.len(),
                errors_string_slice
            );

            let errors_evolved = extract_errors_optimized(text_that_was_read.as_str());
            println!(
                "[Evolved] Found {} errors: {:#?}",
                errors_evolved.len(),
                errors_evolved
            );
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
            //errors = extract_errors(text_that_was_read.as_str());
            // Because as we hit the "}", text_that_was_read is going out of scope, but errors being a Vec<&str> exists and each values inside of it would point to non existing values
            errors = extract_errors_fix(text_that_was_read.as_str());
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }

    println!("Found {} errors: {:#?}", errors.len(), errors);
}

// #07.79
fn writing_data_to_file_process() {
    println!("WRITING DATA TO FILE:");

    match fs::read_to_string("logs.txt") {
        Ok(text_that_was_read) => {
            let errors_string_slice = extract_errors_fix(text_that_was_read.as_str());

            // Since fs::write return a Result, we're going to have imbricated match, which is going to be messy, but we'll fix it afterwards
            match fs::write("errors.txt", errors_string_slice.join("\n")) {
                Ok(..) => println!("Wrote errors.txt"),
                Err(writing_to_file_failed) => {
                    println!("Failed to write to file: {}", writing_to_file_failed);
                }
            }
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed);
        }
    }
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

fn extract_errors_optimized(text: &str) -> Vec<&str> {
    text.lines()
        .filter(|line| line.starts_with("ERROR"))
        .collect()
}

// While this new function (returning a Vec<String> instead of Vec<&str>) would fix the problem,
// remember its return also allocates more data into the Heap.
// (it copies the values of text we're interested in into the Heap ~ which can be an issue if the data was lots of GigaBytes in size)

// If our function receives some text and we need to return text, should we always return a String?
// => Depends!
// Returning a String required extra allocations on the Heap
// We would have been fine returning &str if we didn't expect it to outlive the input text
fn extract_errors_fix(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut result = Vec::new();

    for line in split_text {
        if line.starts_with("ERROR") {
            result.push(line.to_string());
        }
    }
    result
}

fn extract_errors_fix_optimized(text: &str) -> Vec<String> {
    text.lines()
        .filter(|line| line.starts_with("ERROR"))
        .map(|line| line.to_string())
        .collect()
}
