use std::{fs, sync::{Mutex, Once}};

use chrono::NaiveDate;
use rusqlite::Connection;

use crate::{database::rusqlite_impl::{add_category_sqlite, add_transaction_sqlite, get_one_category_sqlite, get_one_transaction_sqlite, get_transaction_sqlite, get_transactions_in_range_sqlite, remove_transaction_sqlite, update_transaction_sqlite}, models::{category::Category, request::transaction_in_range_request_model::TransactionInRangeRequestModel, transaction::Transaction}};

static INIT: Once = Once::new();
static CONN: Mutex<Option<Connection>> = Mutex::new(Option::None);

pub fn initialize() {
    INIT.call_once(|| {
        let mut conn_opt = CONN.lock().unwrap();
        let conn = Connection::open_in_memory().unwrap();
        let sql_init = fs::read_to_string("./src/tests/test_db.db.sql").unwrap();
        let _ = conn.execute_batch(&sql_init);
        *conn_opt = Some(conn);
    });
}

macro_rules! create_test {
    ($test_name:ident,$sql_create_func_name:ident,$sql_get_func_name:ident,$type:ident,$($field:ident;$value:expr),*) => {
        #[test]
        fn $test_name (){
            initialize();
            let conn_opt = CONN.lock().unwrap();
            let conn = conn_opt.as_ref().unwrap();

            let item = $type {
                id:0,
                $($field:$value),*
            };

            let result = $sql_create_func_name(conn,item);
            
            let mut id = 0;
            match result {
                Ok(item_id) => id = item_id,
                Err(error) => assert!(false,"Error during sql execution \n {error}"),
            };
            let retrieved_object = $sql_get_func_name(conn, &id.to_string()).unwrap().unwrap();

            println!("retrieved object id: {id}");
            $(assert_eq!($value,retrieved_object.$field));*
        }
    };
}

create_test!(create_category,add_category_sqlite,get_one_category_sqlite,Category,
name;"example".to_string());

#[test]
fn create_transaction(){
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();

    let transaction = Transaction {
        amount: "40.00".to_string(),
        id: 0,
        category_id: 2,
        transaction_date: "09/27/2024".to_string(),
        name: "item".to_string(),
        recurring: false,
        cycle: None,
        day_of_month: None,
        day_of_week: None
    };

    let result = add_transaction_sqlite(conn, transaction);

    let mut id = 0;
    match result {
        Ok(item_id) => id = item_id,
        Err(error) => assert!(false,"Error during sql execution \n {error}"),
    };
    let retrieved_object = get_one_transaction_sqlite(conn, &id.to_string()).unwrap().unwrap();

    println!("retrieved object id: {id}");
    assert_eq!("40.00",retrieved_object.amount);
    assert_eq!(2,retrieved_object.category_id);
    assert_eq!("09/27/2024",retrieved_object.transaction_date);
    assert_eq!("item",retrieved_object.name);
    assert_eq!(false,retrieved_object.recurring);
    assert_eq!(None,retrieved_object.cycle);
    assert_eq!(None,retrieved_object.day_of_month);
    assert_eq!(None,retrieved_object.day_of_week);
    
}

macro_rules! get_one_test {
    ($x:ident,$y:ident,$z:expr) => {
        #[test]
        fn $x(){
            initialize();
            let conn_opt = CONN.lock().unwrap();
            let conn = conn_opt.as_ref().unwrap();
            let response = $y(conn, stringify!($z));

            match response {
                Ok(result) => assert_eq!($z,result.unwrap().id),
                Err(error) => assert!(false,"Error during sql execution \n {error}"),
            }
        }
    };
}

get_one_test!(get_one_transaction,get_one_transaction_sqlite,20);
get_one_test!(get_one_category,get_one_category_sqlite,2);

#[test]
fn update_transaction(){
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();
    
    let transaction = Transaction {
        amount: "40.00".to_string(),
        id: 12,
        category_id: 2,
        transaction_date: "09/27/2024".to_string(),
        name: "item".to_string(),
        recurring: false,
        cycle: None,
        day_of_month: None,
        day_of_week: None
    };

    let result = update_transaction_sqlite(conn, transaction);

    let id = 12;
    match result {
        Ok(item_id) => println!("updated object id: {item_id}"),
        Err(error) => assert!(false,"Error during sql execution \n {error}"),
    };
    let retrieved_object = get_one_transaction_sqlite(conn, &id.to_string()).unwrap().unwrap();


    assert_eq!("40.00",retrieved_object.amount);
    assert_eq!(2,retrieved_object.category_id);
    assert_eq!("09/27/2024",retrieved_object.transaction_date);
    assert_eq!("item",retrieved_object.name);
    assert_eq!(false,retrieved_object.recurring);
    assert_eq!(None,retrieved_object.cycle);
    assert_eq!(None,retrieved_object.day_of_month);
    assert_eq!(None,retrieved_object.day_of_week);

}

#[test]
fn get_all_transactions(){
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();

    let response = get_transaction_sqlite(conn);

    match response {
        Ok(result) => assert!(result.len() > 0, "Incorrectly returned 0 items"),
        Err(error) => assert!(false,"Error during sql execution \n {error}"),
    }
}

#[test]
fn remove_transaction(){
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();

    let transaction = Transaction {
        amount: "40.00".to_string(),
        id: 14,
        category_id: 2,
        transaction_date: "09/27/2024".to_string(),
        name: "item".to_string(),
        recurring: false,
        cycle: None,
        day_of_month: None,
        day_of_week: None
    };

    let response = remove_transaction_sqlite(conn, transaction);

    match response {
        Ok(_result) => assert!(get_one_transaction_sqlite(conn, "14").unwrap().is_none()),
        Err(error) => assert!(false,"Error during sql execution \n {error}"),
    }
}

#[test]
fn get_transaction_in_range(){
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();

    let transaction_range = TransactionInRangeRequestModel{
        start_date: "2024-09-01".to_string(),
        end_date: "2024-09-30".to_string(),
    };

    let response = get_transactions_in_range_sqlite(conn, &transaction_range);

    match response {
        Ok(result) => for transaction in result{
            let transaction_date = NaiveDate::parse_from_str(&transaction.transaction_date, "%Y-%m-%d").unwrap();
            let range_start = NaiveDate::parse_from_str(&transaction_range.start_date, "%Y-%m-%d").unwrap();
            let range_end = NaiveDate::parse_from_str(&transaction_range.end_date, "%Y-%m-%d").unwrap();

            assert!(transaction_date >= range_start, "transaction date is '{transaction_date}' and range start is '{range_start}'");
            assert!(transaction_date <= range_end,"transaction date is '{transaction_date}' and range end is '{range_end}'");
        },
        Err(error) => assert!(false,"Error during sql execution \n {error}"),
    }
}