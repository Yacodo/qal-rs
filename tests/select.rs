extern crate qal;

use qal::Connector;
use qal::types::*;

use qal::hr::Hr;

#[test]
fn select_all(){
    let hr = Hr{};

    assert_eq!(
        hr.select("my_table").to_string(),
"SELECT
\t*
FROM
\tmy_table"
    );
}

#[test]
fn select_from_two_tables(){
    let hr = Hr{};
    let mut select = hr.select("xyz");
    select.from("abc");

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\txyz,
\tabc"
    );


    let mut select = hr.select("abc");
    select.from("xyz");

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
    let hr = Hr{};
    let mut select = hr.select("xyz");

    select.from(Table::Alias("abc", "t2"));

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\txyz,
\tabc AS t2"
    );

    let mut select = hr.select(Table::Alias("abc", "t1"));

    select.from("xyz");

    assert_eq!(
        select.to_string(),
"SELECT
\t*
FROM
\tabc AS t1,
\txyz"
    );
}

#[test]
fn select_multiples_columns(){
    let hr = Hr{};

    let table = Table::Alias("my_table", "t");
    let mut select = hr.select(table);

    // let columns = select.columns();
    //     columns.add();
    //     columns.add();

}
