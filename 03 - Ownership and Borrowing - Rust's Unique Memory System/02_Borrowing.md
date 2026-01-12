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
