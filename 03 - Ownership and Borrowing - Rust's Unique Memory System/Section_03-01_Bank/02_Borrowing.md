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
- The *Borrowing* system allows us to create **reference** to a value: it allows us to look at a value without **moving** it.
  - Anytime you want to make a value, and then use that value in several locations (passing that value of as a function parameter, refering to it in a struct, or anything like that), 
  having to manually change the owner of that value all the time would be really tedious (an example of that at the end of *01_Ownership.md*).
  - ... So a good solution would be, whenever you have a value you want to use in several locations, is to use a **REFERENCE**
- To make a reference, we're using the "&" operator (its behaviour has sightly different variations depending on where and how you're using it in your code)

```
fn print_account(account: &Account) { // <= here
    println!("{:#?}", account);
}
```
- **& operator**  being used on a **type**.
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

- `3`: You can create many read-only (aka **immutable**) reference to a value. These refs can all exist at the same time.
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

Reason being, when **moving** the value to another owner, the references will then point/reference their original owner's value, which in this case is a "NO VALUE".
