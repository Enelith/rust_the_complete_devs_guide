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

## Understanding String
### CPU Memory: The Stack and Heap
Whenever you run a Rust program, you're going to have three different areas in your computer's memory:
- *Area 1*: the Stack
- *Area 2*: the Heap
- *Area 3*: the Data Segment, the Rodata Segment (Read-Only Data Segment), or the Static Segment (different way to call it)

Note: Lots of online resources don't even refer to the third area at all, and just ignore it. 

#### 1) the Stack
Fast but limited size (2-8MB)

#### 2) the Heap
Slow but can grow a lot of data

#### 3) the Data Segment
Stores literal values that we write into our code 

*Example:*
```
let num = 45;
let color = "red";
```

Let's take a (simplified) look on how the **Stack** and the **Heap** work together to store some data.
```
let nums = vec![1, 2, 3, 4, 5];
```
There's a **very common pattern** in Rust:
- The **Stack** stores metadata about a datastructure
```
Vec Struct {
    pointer to values: ..., // <= Pointing into the Heap
    length: 5,
    capacity: 7
}
```
- The **Heap** stores the data itself
``` 
1, 2, 3, 4, 5
```
- The benefit of this approach, is that it's going to *avoid running out of memory in the Stack 
if the data structure grows to hold a lot of data*

Note that to be more specific, the values contained in the **Heap** are first stored into the **Data Segment**, and then copied into the **Heap**.

There is a Corner Case where the metadata of this Vector would not be stored inside the **Stack**, and instead
would be placed into the **Heap**.

If a data structure owns ANOTHER data structure (nested data structure), the child's metadata will be placed on the **Heap**.
```
let vec_of_nums = vec![
    vec![1, 2, 3, 4, 5]
];
```
- **Stack**:
``` 
'vec_of_nums' Vec {
    pointer to values: ..., // <= Pointing into the Heap
    length: 1,
    capacity: 1
}
```
- **Heap**:
```
Inner Vec {
    pointer to values: ..., // <= Pointing to the values "1, 2, 3, 4, 5" which are ALSO stored into the Heap
    length: 5,
    capacity: 7
}

1, 2, 3, 4, 5
```
- **Data Segment**:
``` 
1, 2, 3, 4, 5
```