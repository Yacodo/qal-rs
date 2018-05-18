extern crate qal;

use qal::Connector;
// use qal::types::*;

use qal::hr::Hr;

#[test]
fn delete_all(){
    let c = Hr{};
    assert_eq!(
        c.delete("my_table").to_string(),
"DELETE FROM
\tmy_table"
    );
}
