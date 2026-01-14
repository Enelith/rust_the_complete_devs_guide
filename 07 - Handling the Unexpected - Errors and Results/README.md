# Handling Errors

## 'fs::read_to_string(<filepath>)' -> The Result enum

Let's imagine we're going to implement the following function `divide`.

```
fn main() {
	println!("{}", divide(10.0, 0.0));
}

fn divide(a: f64, b: f64) -> _ {
	if b == 0.0 {
		// Uh oh, division by 0...
		// Return something to indicate an error occured
	}

	a / b
}
```

**Note:** Dividing something by 0 in Rust is totally fine, you'll get *infinity*... It doesn't actually throw an error, or an exception.

When `b == 0.0`, we'll want to **return something to indicate an error occured**.

#### We're Authoring 'divide'...
2 questions to ask yourself, in order to know what to do:
- Is the error so bad that we couldn't possibly keep running our program, or is it a completely unexpected error ?
	- **Panic**
- Is this an operation that can either succeed or fail ?
	- **Use the Result enum**

Since we actually already considered the possibility that `b = 0`, then we won't technically say it's a completely unexpected error.

Based on the way we wrote the `divide` function, it can either succeed or fail (if `b` is not equal to 0, then we'll succeed, we'll fail otherwise).
<br/>
Therefore, making use of the `Result` enum would be the preferable way to handle this kind of error.

### How do we use the Result enum?

*Actual implementation of Result has some extra stuff*
```
enum Result {
	Ok(value),
	Err
```
**Result** is used when we need to know if something worked or failed.
- `Ok()` variant is used when something went well
- `Err()` variant is used when something went wrong

It is similar in its use to the `Option` enum seen previously:
```
enum Option {
	Some(value),
	None
}
```
**Option** is used when we need to know if a value is present or not.
- `Some()` variant is used when we have a value
- `None` variant is used when there is no value





