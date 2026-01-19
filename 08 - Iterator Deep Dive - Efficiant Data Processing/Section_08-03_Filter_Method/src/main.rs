#[derive(Debug)]
struct Account {
    balance: i32
}

fn main() {
    let accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 },
        Account { balance: -15 },
        Account { balance: 27 },
        Account { balance: -3 }
    ];

    // TODO: Add in a call to the 'filter' iterator adaptor.
    // Find accounts that have a balance less than 0.
    // You can find documentation on 'filter' here:
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter
    let negative_accounts = accounts
        .iter()
        .filter( |account| account.balance < 0)
        .collect::<Vec<_>>();

    println!("Accounts with negative balance: {:#?}", negative_accounts);
}







