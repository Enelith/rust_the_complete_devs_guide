# Section_05-02_Project_Media_Cleanup

## Introduction

That's the follow up from Section_05-01_Project_Media.

### Other Ways of Handling Options

```
    match catalog.get_by_index(9999) {
        Some(value) => {
            println!("Item: {:#?}", value);
        }
        None => {
            println!("Nothing at that index");
        }
    }
```

- Using a Match Statement is the ideal way to figure out if we have `Some` or `None`.
- There are some other ways to figure out if we have `Some` or `None`
  - They're more compact, but have big downsides (program will crash)

--- 
### 1) .unwrap()
``` 
item.unwrap()
```
- If `item` is a `Some`, returns the value in the `Some`.
- If `item` is a `None`, panics!
  - **Use for quick debugging or examples**

---
### 2) .expect(...)
``` 
item.expect("There should be a value here")
```
- If `item` is a `Some`, returns the value in the `Some`.
- If `item` is a `None`, prints the provided debug message and panics!
  - Use when we **want** to crash if something goes wrong

---
### 3) .unwrap_or(&...)
``` 
item.unwrap_or(&placeholder)
```
- If `item` is a `Some`, returns the value in the `Some`.
- If `item` is a `None`, returns the provided default value
  - Use when it makes sense to provide a fallback value

## Getting Started

To run the project, ensure you have Rust installed and run:

```bash
cargo run
```
