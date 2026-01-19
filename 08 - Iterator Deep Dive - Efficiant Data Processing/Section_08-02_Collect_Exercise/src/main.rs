#[derive(Debug)]
struct Account {
    balance: i32
}

fn main() {
    let accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 }
    ];

    // TODO: getting a compiler error around the 'collect' call
    // Remember: 'collect' can be used to gather values into
    // many kinds of data structures. We have to explicitly
    // tell collect what kind of structure we want by adding a
    // type annotation
    let balances = accounts
        .iter()
        .map(|account| account.balance)
        .collect();

    println!("Balances: {:#?}", balances);
}
