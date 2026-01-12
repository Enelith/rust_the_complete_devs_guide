# Ownership

- The goal of Ownership is to limit the ways you can reference and change data
- This limit will reduce the numbers of bugs + make your code easier to understand*

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

#### Rule #1 & #2

- `1`: Every values is *owned* by a **single** variable, struct, vector, etc at a time
- `2`: Reassigning the value to another variable, passing it to a function, putting it into a vector, etc, *moves* the
  value. The old variable can't be used to access the value anymore.

(For the example details, check the video: <br/>
https://www.udemy.com/course/rust-the-complete-developers-guide/learn/lecture/44784613 - *03.25 Visualizing Ownership and Moves*)

**Example 1:**

```
fn main() {
    let bank = Bank::new();
    
    let other_bank = bank;

    println!("{:#?}", bank);
    // Will generate a bug on 'bank': Value used after being moved [E0382]
}
```

In this example,

- `Bank::new()` creates a 'Bank' value.
- `let bank` creates a binding, which will **own** the 'Bank' value.
  ***At any given time, a value can only be own by a SINGLE binding.***
- `let other_bank = bank;` means *"take whatever value the `bank` binding owns (`Bank::new()`), and **move** it to
  the `other_bank` binding"*.
- When **moving** the `Bank::new()` value from the `bank` binding to the `other_bank` binding, the `bank` binding now
  doesn't **own** any value.
- Then, on `println!("{:#?}", bank);`, we're trying to print the value that the `bank` binding owns, but there's no more
  value there.
  **We're not allowed to try to refer to or print out in or in any way access a binding that currently does not have a
  value assigned to it.**

---

**Example 2:**

```
fn main() {
    let account = Account::new(1, String::from("myAccount"));

    let list_of_accounts = vec![account];

    println!("{:#?}", account);
    // Will generate a bug on 'account': 
    // Borrow of moved value: `account` [E0382] 
    // Note: if `Account` implemented `Clone`, you could clone the value
}
```

This case will be a bit more tricky, because we're going to have a value which is going to the owner of another value:

- `Account::new()` creates a 'Account' value`.
- `let account` creates a binding, which will **own** the 'Account' value (we assign the 'Account' value to the
  `account` binding).
- `vec![]` creates a (empty) new Vector value.
- Inside that Vector (`vec![account]`), we take ownership of the 'Account' value:
  <br/>We are now moving the 'Account' value into this Vector at index 0.

| Index | `Vec<Account>` Value |
|:------|:---------------------|
| `0`   | `Account` Value      | 

- We then take this **entire** value (`vec![account]`) and we are assigning it to the `list_of_accounts` binding.

| The binding...     | ... owns the following value... | ... which itself owns the following value at index 0 |
|:-------------------|:--------------------------------|:-----------------------------------------------------|
| `list_of_accounts` | `Vec<Account>` Value            | `0` => `Account` Value                               |

- Similary, trying to do `println!("{:#?}", account);` will print an error (same as before), because `account` doesn't
  own any value at that time.

---

**Example 3:**

```
fn main() {
    let bank = Bank::new();
    
    let list_of_accounts = bank.accounts;

    println!("{:#?}", bank.accounts);
    // Will generate a bug on 'bank.accounts': Value used after being moved [E0382]
}
```

Even trickier:

- `Bank::new()` creates a 'Bank' value (***Reminder: Bank is a struct, and fields in a Struct can also own values***).
  <br/>By doing so, we also initialize a `Vec<Account>` Value which is assigned to the `Bank.accounts` field.

|                  | *'Bank'* Value       |
|:-----------------|:---------------------|
| `accounts` field | `Vec<Account>` Value |

- `let bank` creates a binding, which will **own** the **entire** *'Bank'* Value
- `let list_of_accounts = bank.accounts;` means we're assigning the value owned by the fields `accounts` within the `bank` binding's value (*'Bank'* Value), to another `list_of_accounts` binding.
- Therefore, `bank` binding's value (*'Bank'* Value) looks like this now:

|                  | *'Bank'* Value       |
|:-----------------|:---------------------|
| `accounts` field | **NO VALUE!!**       |
while `list_of_accounts` now owns the `Vec<Account>` Value.

- Lastly, `println!("{:#?}", bank.accounts);` will be looking inside the `bank` binding's value (*'Bank'* Value), specifically at the `accounts` field. But again, no value, and we get an err. 

---

**Example 4:**

```
fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn print_holder(holder: String) {
    println!("{}", holder);
}

fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    print_holder(account.holder);
    
    print_account(account);
    // Will generate a bug on 'account': Value used after being moved [E0382]
}
```
Partially moved Values:
- Likewise, in this case, because the ownership of `account.holder` binding's value was moved into the `print_holder` function, <br/>
the remaining object `account` is partially incomplete, and Rust will also generate an error.

#### Given the ownership system, how do we write useful code ?
- *Option #1:* Manually move values back and forth between different owners (tedious way)
- *Option #2:* Use the borrowing system

An example of Option #1 would be for our print function to return the variable and reassign it to the original one:
```
fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account
}

fn main() {
    let mut account = Account::new(1, String::from("myAccount"));
    
    account = print_account(account);
    account = print_account(account);
    
    println!("{:#?}", account);
}
```
However, we can quickly understand it's going to be tedious, thus comes the **Borrowing* system.
