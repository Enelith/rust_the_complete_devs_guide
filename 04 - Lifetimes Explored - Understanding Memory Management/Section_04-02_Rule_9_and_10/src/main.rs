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

fn make_and_print_account() -> &Account { // <= Error
    let account = Account::new(1, String::from("myAccount"));

    println!("{:#?}", account);

    &account // <= Supposed Error (?)

    // HERE!
}

fn main() {
    let account_ref = make_and_print_account();

    println!("{}", account_ref.balance);
}