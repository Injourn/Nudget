use chrono::{Datelike, Days, Months, NaiveDate, Weekday};

use crate::models::cycle::Cycle;


pub(crate) fn get_dates_between(start_date:&NaiveDate,end_date:&NaiveDate,cycle:&Cycle,value:u32) -> Vec<NaiveDate>{
    match cycle {
        Cycle::MONTHLY => get_dates_between_monthly(start_date, end_date, value),
        Cycle::WEEKLY => get_dates_between_weekly(start_date,end_date, value),
        Cycle::BIWEEKLY => todo!(),
    }
}

pub fn get_dates_between_monthly(start_date:&NaiveDate,end_date:&NaiveDate,value:u32) -> Vec<NaiveDate>{
    let mut curr_date = get_succeeding_day_of_month(start_date, value);
    let mut date_vec = Vec::new();
    while curr_date < *end_date {
        date_vec.push(curr_date.clone());
        curr_date = curr_date.checked_add_months(Months::new(1)).unwrap();
    }
    date_vec
}

pub fn get_dates_between_weekly(start_date:&NaiveDate,end_date:&NaiveDate,value:u32) -> Vec<NaiveDate>{
    let mut curr_date = get_succeeding_day_of_week(start_date, value);
    let mut date_vec = Vec::new();
    while curr_date < *end_date {
        date_vec.push(curr_date.clone());
        curr_date = curr_date.checked_add_days(Days::new(7)).unwrap();
    }
    date_vec
}

pub fn get_succeeding_day_of_month(start_date:&NaiveDate,value:u32) -> NaiveDate {
    let day_of_month = start_date.day();
    let return_date :NaiveDate;
    if day_of_month > value {
        return_date = start_date.checked_add_months(Months::new(1))
        .expect("failed to retrieve date").with_day(value).unwrap();
    }
    else {
        return_date = start_date.with_day(value).unwrap();
    }

    return_date
}

pub fn get_succeeding_day_of_week(start_date:&NaiveDate,value:u32) -> NaiveDate {
    let dt = start_date.and_hms_opt(0, 0, 0).unwrap();
    let day_of_week = modify_weekday_values(&dt.weekday());
    let day_difference ;
    if day_of_week > value {
        day_difference = value + 7 - day_of_week;
    } else {
        day_difference = value - day_of_week
    }

    dt.checked_add_days(Days::new(day_difference as u64)).unwrap().into()
}

fn modify_weekday_values(weekday:&Weekday) -> u32{
    match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}
