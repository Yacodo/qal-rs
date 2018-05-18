extern crate qal;

use qal::Connector;
use qal::types::*;

use qal::hr::Hr;

#[test]
fn select_all(){
    let c = Hr{};

    assert_eq!(
        c.select("my_table").to_string(),
"SELECT
\t*
FROM
\tmy_table"
    );
}

#[test]
fn select_from_two_tables(){
    let c = Hr{};
    let mut select = c.select("xyz");
    select.from("abc", Columns::All);

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\txyz,
\tabc"
    );


    let mut select = c.select("abc");
    select.from("xyz", Columns::All);

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\tabc,
\txyz"
    );
}

#[test]
fn select_from_two_tables_one_alias(){
    let c = Hr{};
    let mut select = c.select("xyz");

    select.from(
        Table::Alias("abc", "t2"),
        Columns::All
    );

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\txyz,
\tabc AS t2"
    );

    let mut select = c.select(Table::Alias("abc", "t1"));

    select.from("xyz", Columns::All);

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\tabc AS t1,
\txyz"
    );
}
