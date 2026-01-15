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

### Strings, String Refs and String Slices
- **String**:
```
let color = String::from("red");
```
When running this line of code, initially, 
- The **Data Segment** will already have stored the literal String "red" inside (because we wrote `red` in our code)
- Then when creating the String, something will be stored in the **Stack** and the **Heap** (similar to when we created the Vector earlier)
  - In the **Stack**, we're going to have a *struct* created
  - At the same time, Rust is going to automatically copy that text data out of the **Data Segment** and store it inside the **Heap**
```
String struct {
    pointer to text in heap: ..., // => Point to "red" inside the Heap
    length of string in heap: 3,
    capacity of string in heap: 3
}
```

Knowing this, what does it really communicate to us? What does this tell us? Why should we care that something is stored in that Stack or that Heap?

It turns out the location where we store these different things (Stack or Heap) makes a huge difference when it comes down differentiating between those 3 different types (*String*, *&String*, and *&str*) 

--- 
- **&String**:
```
let color = String::from("red");
let color_red = &color; // => Will create a reference to the String "red" (&String)
```
When running this code, 
- The **Data Segment** will already have stored the literal String "red" inside (because we wrote `red` in our code)
- We're going to create a *String struct* inside the **Stack** (like before)
```
String struct {
    pointer to text in heap: ..., // => Point to "red" inside the Heap
    length of string in heap: 3,
    capacity of string in heap: 3
}
```
- We're going to copy some data from the **Data Segment** into the **Heap** ("red")
- Then we're trying to have a reference to that `color` variable (`&color`)
  - So that's going to create a reference to the String value inside the **Stack**
    - That reference will point to the *String struct* inside the **Stack**

--- 

- **&str** ~ String Slice:
```
let name = "me";
```
This `&str` refers to a *String slice*.

When we create a String slice by using this kind of expression, we are **NOT** going to use the **Heap** at all!

When running this code, 
- Because we're writing a String literal, some data will be placed inside the **Data Segment**
  - **Data Segment** contains `me`
- We're also going to have a *struct* inside the **Stack**, but this one is only going to have 2 fields
```
&str {
    pointer to text: ..., // => Point directly to "red" inside the Data Segment
    length of string: 2
}
```
Here's the big difference when using a *String slice*: the pointer to the text does not have to involve a **Heap** allocation.
<br/>
Instead, it can just point directly to the data inside our **Data Segment** (so kind of bypassing the **Heap** entirely here).

Another common way to make a *String slice* as well:
```
let color = String::from("red");
let c = color.as_str();
```
`color.as_str()` will also create a *String slice*.

- `red` will be stored in the **Data Segment**
- The `red` from **Data Segment** will be copied into the **Heap**
- We're going to create a *String struct* inside the **Stack** (like before)
```
String struct {
    pointer to text in heap: ..., // => Point to "red" inside the Heap
    length of string in heap: 3,
    capacity of string in heap: 3
}
```
- Then on the next line of code `color.as_str()`, that's where we're going to create our *String slice*
  - Once again, a *struct* will be created inside the **Stack** representing that *String slice*, but this time around, it will not point to some data inside the **Data Segment**.
  - Instead, it's going to be pointing at some data in the **Heap** that is owned by some other String.
```
&str {
    pointer to text: ..., // => Also point to "red" inside the Heap
    length of string: 3
}
```

Now, the big question: what does it really matter? Like what's the point of all these different variations?
<br/>
And perhaps the biggest question: why is there a *String slice* `&str` as a type at all?
