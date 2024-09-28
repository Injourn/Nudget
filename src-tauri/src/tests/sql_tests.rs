use std::{fs, sync::{Mutex, Once}};

use rusqlite::Connection;

use crate::{database::rusqlite_impl::{add_transaction_sqlite, get_one_transaction_sqlite}, models::transaction::Transaction};

static INIT: Once = Once::new();
static mut CONN: Mutex<Option<Connection>> = Mutex::new(Option::None);

pub fn initialize() {
    INIT.call_once(|| {
        let mut conn_opt = unsafe { CONN.lock().unwrap() };
        let conn = Connection::open_in_memory().unwrap();
        let sql_init = fs::read_to_string("./test_db.db.sql").unwrap();
        let _ = conn.execute_batch(&sql_init);
        *conn_opt = Some(conn);
    });
}

#[test]
fn create_transaction(){
    initialize();
    let conn_opt = unsafe { CONN.lock().unwrap() };
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
        Err(_error) => assert!(false,"Error during sql execution"),
    };
    let retrieved_object = get_one_transaction_sqlite(conn, &id.to_string()).unwrap();

    assert_eq!("40.00",retrieved_object.amount);
    assert_eq!(0,retrieved_object.id);
    assert_eq!(2,retrieved_object.category_id);
    assert_eq!("09/27/2024",retrieved_object.transaction_date);
    assert_eq!("item",retrieved_object.name);
    assert_eq!(false,retrieved_object.recurring);
    assert_eq!(None,retrieved_object.cycle);
    assert_eq!(None,retrieved_object.day_of_month);
    assert_eq!(None,retrieved_object.day_of_week);
    
}