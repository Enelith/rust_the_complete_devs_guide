#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

// TODO: Create a new function called 'print_num_accounts'
// It should take in a Bank as an argument, then print out the number
// of accounts stored in the 'bank.accounts' vector
// To get the number of elements in a vector, you can use the '.len()'
// method like this: 'bank.accounts.len()'
fn print_num_accounts(bank: &Bank) {
    println!("Number of accounts: {}", bank.accounts.len());
}


fn main() {
    let mut bank = Bank::new();
    let account1 = Account::new(1, String::from("me"));
    let account2 = Account::new(1, String::from("me"));

    bank.accounts.push(account1);
    bank.accounts.push(account2);

    // TODO: call 'print_num_accounts' here:
    print_num_accounts(&bank);

    // Notice the existing println statement here! 'main' needs to use the
    // 'bank' value in two locations, so we probably shouldn't move the
    // 'bank' value.
    println!("{:#?}", bank);
}







