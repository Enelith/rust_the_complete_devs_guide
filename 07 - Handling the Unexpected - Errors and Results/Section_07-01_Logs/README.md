# Section_07-01_Logs

## Project Overview

In this project, we're going to:
- Open and parse the logs.txt file
- Extract some useful data from the file
- Make sure we have a robust error handling

### Extract some useful data from the file: Pseudocode for what we want to do
```
function extract_errors(log: string) -> list of strings {
    split log by newline characters into lines
    
    initialize an empty list called result
    
    for each line in lines { 
        if line starts with "ERROR" {
            add the line to the result list
        }
    }
    
    return result list
}
```

Because of the way Rust handle strings (there are several kind of String), it's going to be a little bit more challenging than it would appears to be.

To illustrate this, here's the following temporary function:
```
fn string_test(a: String, b: &String, c: &str) {
    
}
```
Ways to call it from the `main` function would be:
```
string_test(
    String::from("red"),
    &String::from("red"),
    "red"
)
```
or 
```
string_test(
    "red".to_string(),
    &String::from("red"),
    String::from("red").as_str()
)
```
Which shows there are still many different ways to create a String in Rust that will still validate the function signature requirements.