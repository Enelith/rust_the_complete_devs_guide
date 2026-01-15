*(Merge of 01_Ownership.md and 02_Borrowing.md)*

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

---

# Borrowing

Considering the following example:

```
fn print_account(account: Account) {
    println!("{:#?}", account);
}

fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    print_account(account);
    
    println!("{:#?}", account);
    // Will generate a bug on 'account': Value used after being moved [E0382]
}
```

a first taste of the borrowing system is by using references instead of passing values to the function.

```
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    let account_ref = &account;
    
    print_account(account_ref);
    
    println!("{:#?}", account);
    // Everything runs fine
}
```

- The *Borrowing* system allows us to create **reference** to a value: it allows us to look at a value without **moving
  ** it.
    - Anytime you want to make a value, and then use that value in several locations (passing that value of as a
      function parameter, refering to it in a struct, or anything like that),
      having to manually change the owner of that value all the time would be really tedious (an example of that at the
      end of *01_Ownership.md*).
    - ... So a good solution would be, whenever you have a value you want to use in several locations, is to use a *
      *REFERENCE**
- To make a reference, we're using the "&" operator (its behaviour has sightly different variations depending on where
  and how you're using it in your code)

```
fn print_account(account: &Account) { // <= here
    println!("{:#?}", account);
}
```

- **`&` operator**  being used on a **type**.
    - Means: *This argument needs to be a reference to a value*.

```
fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    let account_ref = &account; // <= here
    
    print_account(account_ref);
    
    println!("{:#?}", account);
    // Everything runs fine
}
```

- **& operator**  being used on a **owner of a value**.
    - Means: *I want to create a reference to this value*.

--- 

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

#### Rule #3 & #4

- `3`: You can create many read-only (aka **immutable**) reference to a value. These refs can all exist at the same
  time.
- `4`: You can't move a value while a reference to the value exists.

#### Example of Rule #3

```
fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    let account_ref1 = &account;
    let account_ref2 = &account;
    
    print_account(account_ref1);
    print_account(account_ref2);
    
    println!("{:#?}", account);
    // Everything runs fine
}
```

#### Example of Rule #4

```
fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    let account_ref1 = &account;
    let account_ref2 = &account;
    
    let other_account = account; // <= here
    
    print_account(account_ref1);
    print_account(account_ref2);
    
    println!("{:#?}", account); // <= also here
    // We'll get the following error:
    
    cannot move out of `account` because it is borrowed [E0505]
    move out of `account` occurs here
    Note: if `Account` implemented `Clone`, you could clone the value
}
```

Reason being, when **moving** the value to another owner, the references will then point/reference their original
owner's value, which in this case is a "NO VALUE".

--- 

Manually moving ownership to update something (REALLY TEDIOUS)

```
fn change_account(mut account: Account) -> Account {
    account.balance = 10;
    account
}
 
fn main() {
    let account = Account::new(1, String::from("myAccount"));
    
    account = change_account(account);
    
    println!("{:#?}", account);
}
```

The following works, but

- we first created the `Account` value and gave its ownership to `account`.
- Then by having `account` as an argument of the function `change_account`, the `change_account` function now got
  ownership of the `Account` value.
- As the `change_account` returns an Account, the `Account` value switches back to the original `account` value...
- ... which makes the all thing not very practical. Working but tedious.

A much better and more elegant way to implement something like this (a function which is going to receive some data and
will change it in some way), would be to use a **writeable reference** (also known as **mutable reference**).

## Mutable References

In order to create a mutable reference, you need to use both the `&` operator, and the `mut` keyword.

Example:

```
fn print_account(account: &Account) {
    println!("{:#?}", account);
}

// The "&mut Account" indicates we want to use a mutable reference as a type
fn change_account(account: &mut Account) {
    account.balance = 10;
    
    // It would also be fine to read data using a mutable reference.
    // println!("{:#?}", account.holder);
}
 
fn main() {
    // The "mut" here indicates the owner's value can be changed
    let mut account = Account::new(1, String::from("myAccount"));
    
    // "&mut account" here indicates we're creating a mutable reference
    change_account(&mut account);
    
    println!("{:#?}", account);
}
```

- Mutable refs allow us to read or change a value without moving it.

#### Rule #5 & #6

- `5`: You can make a writeable (mutable) reference to a value *only if* there are no read-only references currently in
  use.
  <br/>**One mutable ref to a value can exist at a time**.
- `6`: You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists.

**Example 1 of Rule #5:**

```
fn main() {
    let mut account = Account::new(1, String::from("myAccount"));

    let account_ref = &account; // <= Immutable reference created here
    
    change_account(&mut account); // <= Error when creating the mutable reference
    
    println!("{:#?}", account_ref.holder);
}
```

In this example, the error you're given is :

- *cannot borrow `account` as mutable because it is also borrowed as immutable [E0502]
  mutable borrow occurs here*.

**Rule #5:** You can make a mutable reference to a value, if there are no read-only reference.

---

**Example 2 of Rule #5:**

```
fn main() {
    let mut account = Account::new(1, String::from("myAccount"));

    let account_ref = &mut account; // <= Mutable reference created here
    
    change_account(&mut account); // <= Error when creating another mutable reference
    
    println!("{:#?}", account_ref.holder);
}
```

Likewise, when more than one mutable reference exists at the same time:

- *cannot borrow `account` as mutable more than once at a time [E0499]
  second mutable borrow occurs here*

**Rule #5:** One mutable ref to a value can exist at a time

---

**Example 3 of Rule #6:**

Writing the following is perfectly ok:

```
fn main() {
    let mut account = Account::new(1, String::from("myAccount"));

    account.balance = 10;

    println!("{:#?}", account);
}
```

but if we change the code as follow, ...

```
fn main() {
    let mut account = Account::new(1, String::from("myAccount"));
    
    let account_ref = &mut account;
    
    account.balance = 10;   // <= Error
    
    change_account(account_ref);

    println!("{:#?}", account);
}
```

... then we'll get an error:

- *cannot assign to `account.balance` because it is borrowed [E0506]
  `account.balance` is assigned to here but it was already borrowed*

Because we created a mutable reference (`let account_ref = &mut account;`),
<br/>and then tried to change the value of balance through the owner (`account.balance = 10;`).

(It doesn kind of echo to the **Rule #12** where it's considered as an unexpected update to data)

**Rule #6:** You can't mutate a value through the owner when any ref (mutable or immutable) to the value exists.

#### Rule #7

- `7`: Some type of values are copied instead of moved (numbers, bool, chars, arrays/tuples with copyable elements).

This rule can also be understand as: *Some types of values like numbers, booleans, etc are going to appear
to **break the rules of ownership**.*

**Example of Rule #7:**

Writing the following is perfectly ok:

```
fn main() {
    let num = 5;

    let other_num = num;

    println!("{} {}", num, other_num);
}
```

Some types of values are **copied** instead of **moved**. (This means they behave more like values in other languages)

| Type        | Description                      |
|:------------|:---------------------------------|
| All numbers | Example: i32, u32, f32, ...      |
| bool        | true/false                       |
| char        | Single characters                |
| Arrays      | If everything inside is copyable |
| Tuples      | If everything inside is copyable |
| References  | Both readable and writeable      |
