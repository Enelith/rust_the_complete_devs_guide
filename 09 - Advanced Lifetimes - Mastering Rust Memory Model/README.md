# Lifetime Annotations

`'a` are refered to as *lifetime annotations*.
<br/>
They are used with functions, structs, enums, and more.

```
struct Account {
    balance: i32,
}

struct Bank<'a> {
    primary_account: &'a Account,
}
```

```
fn longest<'a>(str_a: &'a str, str_b: &'a str) -> &'a str {
    if str_a.length() >= str_b.length() {
        str_a
    } else {
        str_b
    }
}  
```

They are (obviously as the name suggests) tied to *lifetimes*. As a reminder, *lifetime* describe how long some kind of value is going to live before it gets cleaned up automatically as it goes out of scope.

- The goal of those *lifetime annotations* is to help the compiler make sure refs won't outlive the value they refer to.
```
struct Account {
    balance: i32,
}

struct Bank<'a> {
    primary_account: &'a Account,
}

fn make_bank() -> Bank {
    let account = Account { balance: 10 };
    let bank = Bank { primary_account: &account }
    
    bank
} 
```
In this example, the issue here is that at the end of the `make_bank()` function, the `account` variable goes out of scope.

So that value is going to be dropped, while we still have a reference to it (`primary_account: &account`).

This will result as an error, the kind of error we're trying to avoid.
- **Hardest Part:** This will seems like something the compiler should do on its own. Yet we're going to focus as for why we need to do this ourselves.
