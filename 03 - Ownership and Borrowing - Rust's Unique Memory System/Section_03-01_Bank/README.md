# Section_03-01_Bank

This is a Rust project.

In this Section, we will:
- Start working on a new project
- Quickly run into an error
- Spend several (many) videos learning some systems in Rust
- Apply new knowledge to fix the error
- Apply new knowledge to finish the project

## Description
### Bank

| Description                                             | Method or Assoc. Function? | Name  | Args | Return |
|:--------------------------------------------------------|:---------------------------|:------|:-----|:-------|
| `Create a 'Bank' instance`                              | Assoc. Function            | new() | -    | Bank   |
| `Add an account to the list of accounts`                |                            |       |      |        |
| `Calculate the total balance of all accounts`           |                            |       |      |        |
| `Create a Vec containing the summaries of all accounts` |                            |       |      |        |

--- 

### Account
| Description                                                    | Method or Assoc. Function? | Name  | Args                    | Return  |
|:---------------------------------------------------------------|:---------------------------|:------|:------------------------|:--------|
| `Create an 'Account' instance`                                 | Assoc. Function            | new() | id: u32, holder: string | Account |
| `Add the given amount of money to the accounts 'balance'`      |                            |       |                         |         |
| `Remove the given amount of money from the accounts 'balance'` |                            |       |                         |         |
| `Create an account summary as a string and return it`          |                            |       |                         |         |


## 3 new systems
`Ownership`, `Borrowing`, `Lifetimes`: 
- Three connected systems
- Tough to understand, but represent 90% of the difficulty of Rust
- Dramatically change the way you will design and write code (compared to other languages)

### 12 Rules
| System        | Rules | Description                                                                                                                                                          |
|:--------------|:------|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Ownership** | `1`   | Every values is 'owned' by a single variable, struct, vector, etc at a time                                                                                          |
|               | `2`   | Reassigning the value of another variable, passing it to a function, putting it into a vector, etc, *moves* the value. The old variable can't be used anymore        |
| **Borrowing** | `3`   | You can create many read-only references to a value that exist at the same time                                                                                      |
|               | `4`   | You can't move a value while a reference to the value exists                                                                                                         |
|               | `5`   | You can make a writeable (mutable) reference to a value *only if* there are no read-only references currently in use. One mutable ref to a value can exist at a time |
|               | `6`   | You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists                                                                   |
|               | `7`   | Some type of values are *copied* instead of moved (numbers, bool, chars, arrays/tuples with copyable elements)                                                       |
| **Lifetimes** | `8`   | When a variable goes out of scope, the value owned by it is *dropped* (cleaned up in memory)                                                                         |
|               | `9`   | Values can't be dropped if there are still active references to it                                                                                                   |
|               | `10`  | References to a value can't outlive the value they refer to                                                                                                          |
| -             | `11`  | **These rules will dramatically change how you write code (compared to other languages)**                                                                            |
| -             | `12`  | **When in doubt, remember that Rust wants to minimize unexpected updates to data**                                                                                     |

### Ownership
- The goal of Ownership is to limit the ways you can reference and change data
- This limite will reduce the numbers of bugs + make your code easier to understand*


## Usage
Run the project using:
```bash
cargo run
```
