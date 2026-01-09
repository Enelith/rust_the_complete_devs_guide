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
            balance: 0,
            holder,
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

fn print_account(account: Account) {
    println!("Account: {:#?}", account);
}

fn main() {
    let bank = Bank::new();
    let account = Account::new(1, String::from("myAccount"));

    println!("Bank: {:#?}", bank);

    print_account(account);
    // Let generate a bug: "Value used after being moved"
    print_account(account);
}
