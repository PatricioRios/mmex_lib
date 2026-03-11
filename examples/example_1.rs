use std::path::Path;
use mmex_lib::domain::TransactionId;
use mmex_lib::MmexContext;

fn main() {
    let ctx =
        MmexContext::open(
            Path::new("personal_finance.mmb"),
            None).expect("Failed to open database");
    

    let transaction = ctx
        .transactions()
        .get_transaction_by_id(TransactionId { v1: 1772139309735031 }).unwrap().unwrap();

    println!("Transaction with ID 1772139309735031: {:#?}", transaction);

    let split_transactions = ctx
        .transactions()
        .get_splits_for_transaction(TransactionId { v1: 1772139309735031 }).unwrap();

    println!("Splits for Transaction ID 1772139309735031:");
    for split in split_transactions {
        println!("{:#?}", split);
    }
}