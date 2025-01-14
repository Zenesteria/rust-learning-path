



#[derive(Debug)]
struct Account{
    id:u32,
    balance:i32,
    holder:String
}
impl Account{
    fn new(id:u32, holder:String) -> Self{
        Account {
            id,
            holder,
            balance:0
        }
    }
}



#[derive(Debug)]
struct Bank{
    accounts:Vec<Account>,
}

impl Bank{
    fn new() -> Self{
        Bank {
            accounts:vec![]
        }
    }
}

fn print_account(account:&Account){
    println!("{:#?}", account)
}

fn change_account(account:&mut Account, balance:i32){
    account.balance = balance; 
}

fn main() {
    let mut account = Account::new(2, String::from("Adebisi"));
    let acc_ref = &account;
    change_account(&mut account, 200);
    print_account(&account);

}
