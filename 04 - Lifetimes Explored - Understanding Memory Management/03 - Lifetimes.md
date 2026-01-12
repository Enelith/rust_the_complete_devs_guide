# Lifetimes

### Terminology:
- Lifetimes: Refers to how long an owner/reference exists.
- Generic Lifetimes/Lifetime Annotations: Extra syntax added to clarify the relationship between different lifetimes.

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
| -             | `12`  | **When in doubt, remember that Rust wants to minimize unexpected updates to data**                                                                                                 |

### Important Reminder for the following
- `1`: Every values is *owned* by a **single** variable, struct, vector, etc at a time

### Rules #8, #9 and #10
- `8`: When a variable goes out of scope, the value owned by it is *dropped* (cleaned up in memory)
- `9`: Values can't be dropped if there are still active references to it
- `10`: References to a value can't outlive the value they refer to

### Rule #8

**Example of Rule #8:**

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


### Rule #9 & Rule #10

**Example:**

```
fn make_and_print_account() -> &Account { // <= Adding a return type annotation
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
