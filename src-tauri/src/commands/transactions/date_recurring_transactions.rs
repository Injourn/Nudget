use chrono::{Datelike, NaiveDate};

use crate::models::response::transaction_response_model::TransactionResponseModel;



pub(crate) fn date_recurring_transactions(transaction_vector: &mut Vec<TransactionResponseModel>, start_date: &str){
    let transaction_iter = transaction_vector.iter_mut();
    for transaction in transaction_iter {
        if transaction.recurring{
            let curr_date = NaiveDate::parse_from_str(start_date,"%Y-%m-%d").expect("failed to parse date");
            let date = NaiveDate::parse_from_str(&transaction.transaction_date,"%Y-%m-%d").expect("failed to parse date");
            let ret_date = date.with_month(curr_date.month()).expect("failed to parse date");
            transaction.transaction_date = ret_date.format("%Y-%m-%d").to_string();
        }
    }
}