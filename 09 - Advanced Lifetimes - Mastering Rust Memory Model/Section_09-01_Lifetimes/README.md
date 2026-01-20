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