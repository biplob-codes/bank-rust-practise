#[derive(Debug)]
struct Account {
    id: u32,
    holder: String,
    balance: i32,
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
fn print_accnt(ac: Account) -> Account {
    print!("{:#?}", ac);
    ac
}
fn main() {
    println!("Hello, world!");
    let mut ac = Account::new(1, String::from("Itachi Uchiha"));
    ac = print_accnt(ac);
    println!("Here is another time: {:#?}", ac)
}
