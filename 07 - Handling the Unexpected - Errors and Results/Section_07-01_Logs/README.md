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

### Why is there &String and &str?
Both provide a read-only reference to text data.

#### Reason #1
```
let color = "red";
```
The first reason we have string slices is that it allows us to refer to some text in specifically the **Data Segment**, without allocating any memory in the **Heap**.
<br/>

*`&str` lets you refer to text in the data segment without a heap allocation*

So it's slightly more performant when we are referring to string literals.

If we didn't have string slice, we would have to write it as follows (with everything in implies):
```
let color = String::from("red");
let color_ref = &color;
```
#### Reason #2
```
let color = String::from("blue");
let portion = &color[1..4];
```
The other reason we have string slices is that it allows us to slice, or kind of take a portion of some text that has already been placed into the **Heap**.

*`&str` lets you 'slice' (take a portion) of text that is already in the **Heap***

To quickly illustrate this,
```
let color = String::from("blue");
```
- `blue` will first be placed inside the **Data Segment**, then copied into the **Heap**.
- A *String struct* will be placed in the **Stack**, 
  - and its `pointer to text in heap` will point to the value `blue`.
  - More specifically, it will point to the first character `B` (`B`, `L`, `U`, `E`).
```
String struct {
    pointer to text in heap: ..., // => Point to "B" inside the Heap
    length of string in heap: 3,
    capacity of string in heap: 3
}
```
By writing:
```
let color = String::from("blue");
let portion = &color[1..4]; // => Starting from the 1st character, go up to, but not including 4 
```
- A *&str struct* will automatically be created and placed inside the **Stack**, 
  - but its `pointer to text` will now point to the character at index 1 of the value stored in the **Heap**
  - Aka, it will point to a portion of text starting from the letter `L` (`L`, `U`, `E`)

So this string slice can be used to point at just a portion of a string that is owned by some other data structure in our **Stack**, or in this case, a string.

If we didn't have a string slice, if **we still want to get a read-only ref to the characters "lue"**, we would have to write it as follows to get an equivalent behavior we can imagine:
```
let color = String::from("blue");
let portion = String::from(
    color.skip(1) // this function doesn't exist, but it gives the idea
);
let portion_ref = &portion;
```

### When do we actually use all these kind of things?

#### String
```
let color = String::from("red")
```
- Use anytime we want ownership of text
- Use anytime we want text that can grow or shrink

#### &String
```
let color = String::from("red")
let color_ref = &color
```
- Rarely (purposelly) used!
- Rust will automatically turn `&String` into `&str` for you
- Another reason is that string slices `&str` kind of have a more immediate sign to developers.
  - *You look at this, and you know you have a read-only reference to a string. Alright.*

#### &str
```
let color = String::from("red");
let c = color.as_str();
```
- Use anytime you don't want to take ownership of text
- Use anytime you want to refer to a **portion** of a string owned by something else

### Summary

| Name    | When to use                                                                   | Uses memory in...  | Notes                                                      |
|:--------|:------------------------------------------------------------------------------|:-------------------|:-----------------------------------------------------------|
| String  | `When you want to take ownership of text data`                              | `Stack` and `Heap` |                                                            | 
|         | `When you have a string that might grow or shrink`                            |                    |                                                            | 
| &String | `Usually never`                                                               | `Stack`            | `Rust automatically turns &String into &str for you`       | 
| &str    | `When you want to read all or a portion of some text owned by something else` | `Stack`            | `Refers directly to heap-allocated or data-allocated text` | 