use chrono::{Days, Local, NaiveDate};
use log::info;
use rusqlite::Connection;

use crate::{database::rusqlite_impl::{add_transaction_sqlite, get_most_recent_recurring_transaction, get_recurring_transactions_sqlite}, functions::get_dates_between::get_dates_between_monthly, models::transaction::Transaction};


//TODO: clean up error handling.
pub(crate) fn insert_recurring_transactions(conn: &Connection,
    handle: &tauri::AppHandle){
    info!("Inserting recurring transactions");
    // Get all recurring transactions
    let recurring_transactions = get_recurring_transactions_sqlite(conn, handle).expect("failed to load transactions");
    // For all transactions: 
    for recurring_transaction in recurring_transactions {
        let rt = &recurring_transaction;
        // Get most recent recurring transaction with or fall back with recurring transaction creation date
        let recurring_transaction_id_option = recurring_transaction.recurring_transaction_id.clone();
        let mut latest_transaction: Option<Transaction> = Option::None;
        match recurring_transaction_id_option {
            Some(rci) => {
                latest_transaction = get_most_recent_recurring_transaction(conn, handle, &rci)
                .expect("Error finding the latest transaction.");
            },
            None => {
                // TODO: WARN log because recurring transaction did not exist.
            }
        }
        // Using insertion method, add all recurring transactions up to current date. (if the next date for the
        // transaction exceeds the current date, stop there)
        let dates;
        match latest_transaction {
            Some(t) => {
                let n_transaction_date =
                NaiveDate::parse_from_str(&t.transaction_date.as_str(), "%Y-%m-%d").expect("failed to parse date");
                let start_date = n_transaction_date.checked_add_days(Days::new(1)).unwrap();
                dates = get_dates_for_recurring_transaction(&start_date,&recurring_transaction);
            },
            None => {
                let n_transaction_date =
                NaiveDate::parse_from_str(&recurring_transaction.transaction_date.as_str(), "%Y-%m-%d").expect("failed to parse date");
                dates = get_dates_for_recurring_transaction(&n_transaction_date,&recurring_transaction);
            }
        }
        // include recurring transaction id
        for date in dates {
            let recurring_transaction_id = rt.recurring_transaction_id.clone();
            assert!(recurring_transaction_id != None);
            let new_transaction = Transaction {
                id: 0,
                recurring_transaction_id,
                amount: rt.amount.clone(),
                category_id: recurring_transaction.category_id,
                transaction_date: date.format("%Y-%m-%d").to_string(),
                name: rt.name.clone(),
                recurring: false,
                credit: recurring_transaction.credit,
                cycle: None,
                day_of_month: None,
                day_of_week: None,
                account_id: recurring_transaction.account_id

            };
            let _ = add_transaction_sqlite(conn, new_transaction);
        }
    }

}

fn get_dates_for_recurring_transaction(
    start_date: &NaiveDate,
    transaction: &Transaction
) -> Vec<NaiveDate> {
    let n_now = Local::now().date_naive();
    //TODO: apply to other budget intervals
    get_dates_between_monthly(start_date,&n_now,
        transaction
        .day_of_month
        .or(transaction.day_of_week)
        .unwrap() as u32)
}