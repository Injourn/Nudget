use chrono::NaiveDate;

use crate::{
    functions::get_dates_between::get_dates_between,
    models::response::transaction_response_model::TransactionResponseModel,
};

pub(crate) fn date_recurring_transactions(
    transaction_vector: &mut Vec<TransactionResponseModel>,
    start_date: &str,
    end_date: &str,
) {
    let transaction_iter = transaction_vector.iter_mut();
    for transaction in transaction_iter {
        if transaction.recurring {
            let n_start_date =
                NaiveDate::parse_from_str(start_date, "%Y-%m-%d").expect("failed to parse date");
            let n_end_date =
                NaiveDate::parse_from_str(end_date, "%Y-%m-%d").expect("failed to parse date");
            let date_vector = get_dates_between(
                &n_start_date,
                &n_end_date,
                &transaction.cycle.as_ref().unwrap(),
                transaction
                    .day_of_month
                    .or(transaction.day_of_week)
                    .unwrap() as u32,
            );
            if date_vector.len() > 0 {
                transaction.transaction_date = date_vector[0]
                .format("%Y-%m-%d")
                .to_string();
            } 
            
        }
    }
}
