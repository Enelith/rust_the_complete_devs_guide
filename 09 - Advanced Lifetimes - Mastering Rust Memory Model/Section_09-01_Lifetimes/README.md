# Section_09-01_Lifetimes

```
let languages = vec![
    String::from("rust"),
    String::from("go"),
    String::from("typescript"),
];
```

| Name                   | Description                                     | Args            | Return |
|:-----------------------|:------------------------------------------------|:----------------|:-------|
| `last_language()`      | Returns the last element in the vector          | &[String]       | &str   |
| `next_language()`      | Finds a given language and returns the next one | &[String], &str | &str   |
| `longest()`            | Returns the longer of 2 languages               |  &str, &str     | &str   |

### Next_language

- We're going to make a list of languages, and pass those in as an argument (ex: `languages = [ 'rust', 'go', 'typescript']`)
- We're also going to pass a String Slice (ex: `'go'`)
- Then we're going to try to find where this string occurs inside our list of languages.
- We're then going to return the next language after that (ex: `returns 'typescript'`)

While this seems pretty easy to implement, we're going to face an error which can only be fixed through a *lifetime annotation* `'a`.

We wrote the following code, and as expected, we got an error: 
```
fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");
    println!("Next language: {:#?}", result);
}

// There's an error on "<- &str"
fn next_language(languages: &[String], current: &str) -> &str {
    let mut found = false;

    for lang in languages {
        if found {
            return lang;
        }
        if lang == current {
            found = true;
        }
    }

    languages.last().unwrap()
}
```
The error is located on the `<- &str` (more specifically on the `&`), and says:
```
Missing lifetime specifier [E0106]
```

*Lifetime* is the duration from when we create a variable and initialize it with some value to when, either that variable goes out of scope, or we move the value outside of it.


As a reminder, we saw earlier that there were 12 rules based on three very important notions: **Ownership**, **Borrowing** and **Lifetime**.

### 12 Rules

| System        | Rules | Description                                                                                                                                                                        |
|:--------------|:------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **Ownership** | `1`   | Every values is *owned* by a **single** variable, struct, vector, etc at a time                                                                                                    |
|               | `2`   | Reassigning the value to another variable, passing it to a function, putting it into a vector, etc, *moves* the value. The old variable can't be used to access the value anymore. |
| **Borrowing** | `3`   | You can create many read-only references to a value that exist at the same time                                                                                                    |
|               | `4`   | You can't move a value while a reference to the value exists                                                                                                                       |
|               | `5`   | You can make a writeable (mutable) reference to a value *only if* there are no read-only references currently in use. One mutable ref to a value can exist at a time               |
|               | `6`   | You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists                                                                                 |
|               | `7`   | Some type of values are *copied* instead of moved (numbers, bool, chars, arrays/tuples with copyable elements)                                                                     |
| **Lifetimes** | `8`   | When a variable goes out of scope, the value owned by it is *dropped* (cleaned up in memory)                                                                                       |
|               | `9`   | Values can't be dropped if there are still active references to it                                                                                                                 |
|               | `10`  | References to a value can't outlive the value they refer to                                                                                                                        |
| -             | `11`  | **These rules will dramatically change how you write code (compared to other languages)**                                                                                          |
| -             | `12`  | **When in doubt, remember that Rust wants to minimize unexpected updates to data**    

In this instance, we previously saw the rule `10` which says that *a reference to a value cannot outlive the value that it refers to*.

Our current implementation is as follow, and we're going to pretend the implementation of the method `next_language` is working fine, without errors.

```
fn next_language(languages: &[String], current: &str) -> &str {
    /* Implementation */
}

fn main() {
    let languages = vec![
        String::from("rust"),
        String::from("go"),
        String::from("typescript"),
    ];

    let result = next_language(&languages, "go");
    println!("Next language: {}", result);
}
```
Behind the scenes, here's what happening:
- As our code runs, we start off inside of `main()`.
- We declare a Vector `vec![]` (the original Vector), and assign it to the `languages` binding.
- Then we create a reference to languages (`&languages`), 
and we pass it off to the `next_language()` function (`next_languages(&languages, "go")`).
  - That reference is going to show up inside this function as the `languages` argument (`fn next_language(languages: &[String], ...`)
  - So we could say *we created a reference to a value, and it gets assigned to the `languages` argument, while still pointing to the original Vector*
- The `next_language()` function is going to return a *reference* to a string (`fn next_languages(...) -> &str`)
  - No matter what, this returned reference is pointing at one of the values inside the `languages` arg (`languages: &[String]`)
  - We know this for certain, just by looking at the implementation of our function
    - **(A)**: In this first case, when returning here, we ARE returning a reference to one of the languages inside the `languages` (arg) vector
    - **(B)**: Even if we don't find what we were looking for, if we fall to into this base case scenario, we're STILL returning a reference to one of the strings inside the `languages` (arg) vector
```
/* Implementation */
    let mut found = false;

    for lang in languages {
        if found {
            return lang; // <= Return (A)
        }
        if lang == current {
            found = true;
        }
    }

    languages.last() // <= Return (B)
        .unwrap()
```
- Therefore, we assign the result of the `next_language()` function to the `result` binding, 
we're then going to have another reference, and it's guaranteed to point at one of the different strings contained within the original Vector. 
  - At the same time, when we return from the `next_language()` function, we're going to destroy the `languages` argument (`languages: &[String]`) since it's going out of scope, 
  and we're going to drop that argument and the value assigned to it.
- At the bottom of our `main()` function, we're going to print out whatever `result` is (which again, is going to be a reference to a string contained in the original Vector).

In this scenario, everything works fine, there were no issues, and at no point of time did we ever violate the rules `8`, `9`, and `10`. 

--- 

Now let's have a case where rule `10` is violated.

```
fn next_language(languages: &[String], current: &str) -> &str {
    /* Implementation */
}

fn main() {
    let result;
  
    {
        let languages = vec![
            String::from("rust"),
            String::from("go"),
            String::from("typescript"),
        ];
        
        result = next_language(&languages, "go");
    } // <= 'C': languages goes out of scope, its value is dropped!

    println!("Next language: {}", result);
}
```
In this case, in the `main()` function, we've put the `languages` binding inside an inner scope.
<br/>
We declared the `result` *outside* of this inner scope, and we initialized it *inside* that inner scope. 

- Like before, we're going to create a Vector `vec![]` which we'll assign to the `languages` binding in the `main()` function.
- We're going to create a reference to that vector, and pass it off to the `next_language()` function.
- Inside of `next_language()`, we're going to run through the implementation, where we're going to create another reference to one of the different strings, and then return it.
- And that's going to be assigned to the `result` binding.
- As we return from `next_languages()`, we're also going to have the `languages: &[String]` argument (reference) be cleaned up
  - and at this point of time `C`, `languages` goes out of scope and its value is dropped.
  - *Because* `language` is going to be dropped, the entire original Vector is also going to go away.
    - But the issue here is that we HAVE a reference to that value (`result`), and we can't drop a value while there is a reference to it.

In this scenario, we're going to end up with an error.