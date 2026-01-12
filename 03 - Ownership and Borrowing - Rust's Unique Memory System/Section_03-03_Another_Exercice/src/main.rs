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

fn print_bank(bank: Bank) {
    println!("{:#?}", bank);
}

fn main() {
    let bank = Bank::new();

    // TODO: Write and call a function that will *take ownership* of
    // the Banks's "accounts" field, print it, and return nothing
    print_accounts(bank.accounts);

    // TODO: Once you've finished the to-do, uncomment the 'print_bank' call below
    // When your function + print_bank run, do you think you'll end up getting
    // an error?
    // If so, what error do you think you'd see?

    // UNCOMMENT THIS:
     print_bank(bank);  // <= It won't compile. We'll get the following error "Value used after being moved [E0382]" on the "bank" variable
    // aka: a "PARTIAL MOVE"
}

fn print_accounts(accounts: Vec<Account>) {
    println!("{:#?}", accounts);
}







