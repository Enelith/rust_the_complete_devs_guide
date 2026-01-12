# Section_04-02_Rule_9_and_10

### Important Reminder for the following
- `1`: Every values is *owned* by a **single** variable, struct, vector, etc at a time

### Rules #8, #9 and #10
- `8`: When a variable goes out of scope, the value owned by it is *dropped* (cleaned up in memory)
- `9`: Values can't be dropped if there are still active references to it
- `10`: References to a value can't outlive the value they refer to

---

## Rule #9 & Rule #10

**Example:**

```
fn make_and_print_account() -> &Account { // <= Error
	let account = Account::new(1, String::from("myAccount"));

	println!("{:#?}", account);

	&account // <= Error
}

fn main() {
	let account_ref = make_and_print_account();

	println!("{}", account_ref.balance);
}
```

- Our function `make_and_print_account` now returns a **Reference** (`-> &Account`), ...
- ...which we'll save into the binding `account_ref`, ...
- ...and we'll try to print the `account_ref.balance` value.

On the `-> &Account`, we have an error:
- *this function's return type contains a borrowed value, but there is no value for it to be borrowed from.
<br/>**help:** consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static` (`fn make_and_print_account() -> &static Account {`).
<br/>**help:** instead, you are more likely to want to return an owned value.*

At the return `&account`, we're suppose to get an error, such as the owner goes out of all scopes, while a ref still exists.

From the `// HERE!` point, Rust should delete both the `account` binding and the Account value, but the returned **reference** would still exists (and would refer to nothing).

That's **Rule #9 / #10** quickly explained. 
<br/>Rust is going to make sure that you never attempt to return a reference to a value, that just is going to go out of scope.

--- 
## Usage

Run the project using:

```bash
cargo run
```
