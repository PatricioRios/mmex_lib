use std::path::Path;
use mmex_lib::domain::TransactionId;
use mmex_lib::MmexContext;

fn main() {
    let ctx =
        MmexContext::open(
            Path::new("personal_finance.mmb"),
            None).expect("Failed to open database");


    let accounts = ctx
        .accounts()
        .get_all_accounts()
        .unwrap();

    let account_balance = ctx
        .accounts()
        .get_account_balance(accounts[0].id)
        .unwrap();
    
    println!("Account: {}",accounts.get(0).unwrap().name);
    println!("{:#?}", account_balance);

}