use std::{
    fs,
    sync::{Mutex, Once},
};

use chrono::NaiveDate;
use log::debug;
use rusqlite::Connection;

use crate::{
    database::rusqlite_impl::{
        add_account_sqlite, add_budget_category_sqlite, add_budget_plan_sqlite, add_budget_sqlite,
        add_category_sqlite, add_transaction_sqlite, get_account_summary_in_range_sqlite,
        get_all_accounts_sqlite, get_all_budget_categories_sqlite, get_all_budget_plan_sqlite,
        get_all_budget_sqlite, get_all_categories_sqlite, get_one_account_sqlite,
        get_one_budget_category_sqlite, get_one_budget_plan_sqlite, get_one_budget_sqlite,
        get_one_category_sqlite, get_one_transaction_sqlite, get_transaction_sqlite,
        get_transactions_in_range_sqlite, remove_account_sqlite, remove_budget_category_sqlite,
        remove_budget_plan_sqlite, remove_budget_sqlite, remove_category_sqlite,
        remove_transaction_sqlite, update_account_sqlite, update_budget_category_sqlite,
        update_budget_plan_sqlite, update_budget_sqlite, update_category_sqlite,
        update_transaction_sqlite,
    },
    models::{
        account::Account,
        budget::Budget,
        budget_category::BudgetCategory,
        budget_plan::BudgetPlan,
        category::Category,
        cycle::Cycle,
        request::{
            account_summary_in_range_request::AccountSummaryInRangeRequest,
            transaction_in_range_request_model::TransactionInRangeRequestModel,
        },
        transaction::Transaction,
    },
};

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

            debug!("retrieved object id: {id}");
            $(assert_eq!($value,retrieved_object.$field,"field {} failed to match value", stringify!($field)));*
        }
    };
}

create_test!(create_category,add_category_sqlite,get_one_category_sqlite,Category,
name;"example".to_string());
create_test!(create_transaction,add_transaction_sqlite,get_one_transaction_sqlite,Transaction,
amount; "40.00".to_string(),
category_id; 2,
transaction_date; "09/27/2024".to_string(),
name; "item".to_string(),
recurring; false,
credit; false,
cycle; None,
day_of_month; None,
day_of_week; None,
account_id; None,
recurring_transaction_id; None);
create_test!(create_budget,add_budget_sqlite,get_one_budget_sqlite,Budget,
start_date;"2024-05-15".to_string(),
end_date;"2024-06-15".to_string(),
cycle;Some(Cycle::MONTHLY));
create_test!(create_budget_plan,add_budget_plan_sqlite,get_one_budget_plan_sqlite,BudgetPlan,
    cycle;Cycle::MONTHLY,
    start_date_of_month;Some(15),
    start_date_of_week;None,
    active;false,
    name;"Next".to_string());
create_test!(create_budget_category,add_budget_category_sqlite,get_one_budget_category_sqlite,BudgetCategory,
    category_id; 2,
    flat_amount; "700".to_string(),
    fixed;true,
    percentage_amount;"".to_string()
);
create_test!(create_account,add_account_sqlite,get_one_account_sqlite,Account,
    name; "main".to_string(),
    created_date; "2024-10-23".to_string(),
    currency_type; "$".to_string()
);

macro_rules! get_one_test {
    ($x:ident,$y:ident,$z:expr) => {
        #[test]
        fn $x() {
            initialize();
            let conn_opt = CONN.lock().unwrap();
            let conn = conn_opt.as_ref().unwrap();
            let response = $y(conn, stringify!($z));

            match response {
                Ok(result) => assert_eq!($z, result.unwrap().id),
                Err(error) => assert!(false, "Error during sql execution \n {error}"),
            }
        }
    };
}

get_one_test!(get_one_transaction, get_one_transaction_sqlite, 20);
get_one_test!(get_one_category, get_one_category_sqlite, 2);
get_one_test!(get_one_budget, get_one_budget_sqlite, 1);
get_one_test!(get_one_budget_category, get_one_budget_category_sqlite, 2);
get_one_test!(get_one_budget_plan, get_one_budget_plan_sqlite, 2);
get_one_test!(get_one_account, get_one_account_sqlite, 1);

macro_rules! update_test {
    ($test_name:ident,$sql_update_func_name:ident,$sql_get_func_name:ident,$type:ident,$($field:ident;$value:expr),*) => {
        #[test]
        fn $test_name (){
            initialize();
            let conn_opt = CONN.lock().unwrap();
            let conn = conn_opt.as_ref().unwrap();

            let item = $type {
                $($field:$value),*
            };
            let id = item.id;

            let result = $sql_update_func_name(conn,item);

            match result {
                Ok(_) => debug!("updated object successfully"),
                Err(error) => assert!(false,"Error during sql execution \n {error}"),
            };
            let retrieved_object = $sql_get_func_name(conn, &id.to_string()).unwrap().unwrap();

            debug!("retrieved object id: {id}");
            $(assert_eq!($value,retrieved_object.$field));*
        }
    };
}

update_test!(update_category,update_category_sqlite,get_one_category_sqlite,Category,
id;5,name;"Bills and Subscriptions".to_string());
update_test!(update_transaction,update_transaction_sqlite,get_one_transaction_sqlite,Transaction,
    id;12,
    amount; "40.00".to_string(),
    category_id; 2,
    transaction_date; "09/27/2024".to_string(),
    name; "item".to_string(),
    recurring; false,
    credit; false,
    cycle; None,
    day_of_month; None,
    day_of_week; None,
    account_id; None,
    recurring_transaction_id; None);
update_test!(update_budget,update_budget_sqlite,get_one_budget_sqlite,Budget,
    id;1,
    start_date;"2024-05-15".to_string(),
    end_date;"2024-06-15".to_string(),
    cycle;Some(Cycle::MONTHLY));
update_test!(update_budget_plan,update_budget_plan_sqlite,get_one_budget_plan_sqlite,BudgetPlan,
    id;2,
    cycle;Cycle::MONTHLY,
    start_date_of_month;Some(15),
    start_date_of_week;None,
    active;false,
    name;"Next".to_string());
update_test!(update_budget_category,update_budget_category_sqlite,get_one_budget_category_sqlite,BudgetCategory,
    id;2,
    category_id; 2,
    flat_amount; "700".to_string(),
    fixed;true,
    percentage_amount;"".to_string()
);
update_test!(update_account,update_account_sqlite,get_one_account_sqlite,Account,
    id;1,
    name; "main".to_string(),
    created_date; "2024-10-23".to_string(),
    currency_type; "$".to_string()
);

macro_rules! get_all {
    ($test_name:ident,$func_name:ident) => {
        #[test]
        fn $test_name() {
            initialize();
            let conn_opt = CONN.lock().unwrap();
            let conn = conn_opt.as_ref().unwrap();

            let response = $func_name(conn);

            match response {
                Ok(result) => assert!(result.len() > 0, "Incorrectly returned 0 items"),
                Err(error) => assert!(false, "Error during sql execution \n {error}"),
            }
        }
    };
}

get_all!(get_all_categories, get_all_categories_sqlite);
get_all!(get_all_transactions, get_transaction_sqlite);
get_all!(get_all_budget, get_all_budget_sqlite);
get_all!(get_all_budget_category, get_all_budget_categories_sqlite);
get_all!(get_all_budget_plan, get_all_budget_plan_sqlite);
get_all!(get_all_accounts, get_all_accounts_sqlite);

macro_rules! remove_test {
    ($test_name:ident,$sql_remove_func_name:ident,$sql_get_func_name:ident,$type:ident,$($field:ident;$value:expr),*) => {
        #[test]
        fn $test_name(){
            initialize();
            let conn_opt = CONN.lock().unwrap();
            let conn = conn_opt.as_ref().unwrap();

            let item = $type {
                $($field:$value),*
            };

            let item_id = item.id;

            let response = $sql_remove_func_name(conn, item);

            match response {
                Ok(_result) => assert!($sql_get_func_name(conn, &item_id.to_string()).unwrap().is_none()),
                Err(error) => assert!(false,"Error during sql execution \n {error}"),
            }
        }
    };
}

remove_test!(remove_category,remove_category_sqlite,get_one_category_sqlite,Category,
    id;6,name;"Gas".to_string());
remove_test!(remove_transaction,remove_transaction_sqlite,get_one_transaction_sqlite,Transaction,
    id;14,
    amount; "40.00".to_string(),
    category_id; 2,
    transaction_date; "09/27/2024".to_string(),
    name; "item".to_string(),
    recurring; false,
    credit; false,
    cycle; None,
    day_of_month; None,
    day_of_week; None,
    account_id; None,
    recurring_transaction_id; None);
remove_test!(remove_budget,remove_budget_sqlite,get_one_budget_sqlite,Budget,
    id;2,
    start_date;"2024-05-15".to_string(),
    end_date;"2024-06-15".to_string(),
    cycle;Some(Cycle::MONTHLY));
remove_test!(remove_budget_plan,remove_budget_plan_sqlite,get_one_budget_plan_sqlite,BudgetPlan,
    id;3,
    cycle;Cycle::MONTHLY,
    start_date_of_month;Some(15),
    start_date_of_week;None,
    active;false,
    name;"Next".to_string());
remove_test!(remove_budget_category,remove_budget_category_sqlite,get_one_budget_category_sqlite,BudgetCategory,
    id;3,
    category_id; 2,
    flat_amount; "700".to_string(),
    fixed;true,
    percentage_amount;"".to_string()
);
remove_test!(remove_account,remove_account_sqlite,get_one_account_sqlite,Account,
    id;2,
    name; "main".to_string(),
    created_date; "2024-10-23".to_string(),
    currency_type; "$".to_string()
);

#[test]
fn get_transaction_in_range() {
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();

    let transaction_range = TransactionInRangeRequestModel {
        start_date: "2024-09-01".to_string(),
        end_date: "2024-09-30".to_string(),
    };

    let response = get_transactions_in_range_sqlite(conn, &transaction_range);

    match response {
        Ok(result) => {
            for transaction in result {
                let transaction_date =
                    NaiveDate::parse_from_str(&transaction.transaction_date, "%Y-%m-%d").unwrap();
                let range_start =
                    NaiveDate::parse_from_str(&transaction_range.start_date, "%Y-%m-%d").unwrap();
                let range_end =
                    NaiveDate::parse_from_str(&transaction_range.end_date, "%Y-%m-%d").unwrap();

                assert!(
                    transaction_date >= range_start,
                    "transaction date is '{transaction_date}' and range start is '{range_start}'"
                );
                assert!(
                    transaction_date <= range_end,
                    "transaction date is '{transaction_date}' and range end is '{range_end}'"
                );
            }
        }
        Err(error) => assert!(false, "Error during sql execution \n {error}"),
    }
}

#[test]
fn get_account_summary_in_range() {
    initialize();
    let conn_opt = CONN.lock().unwrap();
    let conn = conn_opt.as_ref().unwrap();

    let example_request = AccountSummaryInRangeRequest {
        account_id: 1,
        start_date: "2024-09-19".to_string(),
        end_date: "2024-09-26".to_string(),
    };

    let response = get_account_summary_in_range_sqlite(conn, &example_request);

    match response {
        Ok(result) => {
            let first_result_option = result;
            assert!(
                first_result_option.is_some(),
                "Did not get a proper response for account summary response"
            );
            let first_result = first_result_option.unwrap();

            assert!(first_result.account_id == 1);
            assert!(
                first_result.debit_transactions == 1768.44,
                "Expected credit transactions to be 1768.44 but was actually {0}",
                first_result.debit_transactions
            );
            assert!(
                first_result.credit_transactions == 6000.00,
                "Expected credit transactions to be 6000.00 but was actually {0}",
                first_result.credit_transactions
            );
        }
        Err(error) => assert!(false, "Error during sql execution \n {error}"),
    }
}
