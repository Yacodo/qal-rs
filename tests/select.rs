extern crate qal;

use qal::{Query, Select};

#[test]
fn print_empty_query(){
    let select = Select::new();

    assert_eq!(
        select.to_string(), // Should print nothing
        "",
    );
}
