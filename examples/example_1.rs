use std::path::Path;
use mmex_lib;
use mmex_lib::MmexContext;

fn main() {
    let ctx =
        MmexContext::open(
            Path::new("personal_finance.mmb"),
            None).expect("Failed to open database");

    let result = ctx.tags().get_all_tags();

    match result {
        Ok(items) => {
            println!("Items:");
            for item in items {
                println!("{:?}", item);
            }
        }
        Err(e) => eprintln!("Error : {}", e),
    }






}