use mmex_lib::*;
use std::path::{self, Path};

fn main() {
    let ctx = MmexContext::open(
        Path::new("/home/patriciorios/Proyectos/mmex_workarea/tests/test_db.mmb"),
        None,
    )
    .unwrap();

    let tagsSetvice = ctx.tags();

    let tags = tagsSetvice.get_all_tags().unwrap();

    for tag in tags {
        print!("Name: {}, Id: {}", tag.name, tag.id.v1);
    }
}
