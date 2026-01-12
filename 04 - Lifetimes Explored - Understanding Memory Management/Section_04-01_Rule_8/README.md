# Section_04-01_Rule_8

### Important Reminder for the following
- `1`: Every values is *owned* by a **single** variable, struct, vector, etc at a time

### Rules #8, #9 and #10
- `8`: When a variable goes out of scope, the value owned by it is *dropped* (cleaned up in memory)
- `9`: Values can't be dropped if there are still active references to it
- `10`: References to a value can't outlive the value they refer to

---

## Rule #8

**Example:**

```
fn make_and_print_account() {
	let account = Account::new(1, String::from("myAccount"));

	println!("{:#?}", account);

	// HERE!
}

fn main() {
	make_and_print_account();
}
```
In this example, at the `// HERE!` comment, after this point, is there any way to refer to the `account` binding, or the Account value?

Answer: No. The instant we return from this function, there's no way we can get access to the `account` binding, or the Account value, even though they still exist in memory.

So because we don't really expect to make use of those things any time after you return from the function, Rust is going to automatically delete the `account` binding, and the Account value along with it.

That's **Rule #8**. 

--- 
## Usage

Run the project using:

```bash
cargo run
```
