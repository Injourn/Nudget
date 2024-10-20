use std::borrow::Borrow;

use chrono::NaiveDate;

use crate::functions::get_dates_between::{
    get_dates_between_monthly, get_dates_between_weekly, get_succeeding_day_of_month,
    get_succeeding_day_of_week,
};

#[test]
fn get_next_day_of_month_simple() {
    let date =
        get_succeeding_day_of_month(NaiveDate::from_ymd_opt(2024, 9, 7).unwrap().borrow(), 19);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 9, 19).unwrap());
}

#[test]
fn get_next_day_of_month_next_month() {
    let date =
        get_succeeding_day_of_month(NaiveDate::from_ymd_opt(2024, 9, 7).unwrap().borrow(), 1);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 10, 1).unwrap());
}

#[test]
fn get_next_day_of_month_equal() {
    let date =
        get_succeeding_day_of_month(NaiveDate::from_ymd_opt(2024, 9, 7).unwrap().borrow(), 7);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 9, 7).unwrap());
}

#[test]
fn get_next_day_of_week_simple() {
    let date = get_succeeding_day_of_week(NaiveDate::from_ymd_opt(2024, 9, 9).unwrap().borrow(), 6);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 9, 14).unwrap());
}

#[test]
fn get_next_day_of_week_next_week() {
    let date = get_succeeding_day_of_week(NaiveDate::from_ymd_opt(2024, 9, 6).unwrap().borrow(), 1);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 9, 9).unwrap());
}

#[test]
fn get_next_day_of_week_equal() {
    let date = get_succeeding_day_of_week(NaiveDate::from_ymd_opt(2024, 9, 5).unwrap().borrow(), 4);
    assert_eq!(date, NaiveDate::from_ymd_opt(2024, 9, 5).unwrap());
}

#[test]
fn get_dates_between_monthly_one() {
    let dates = get_dates_between_monthly(
        &NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
        &NaiveDate::from_ymd_opt(2024, 9, 30).unwrap(),
        15,
    );
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 9, 15).unwrap());
}

#[test]
fn get_dates_between_monthly_mult() {
    let dates = get_dates_between_monthly(
        &NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
        &NaiveDate::from_ymd_opt(2024, 11, 30).unwrap(),
        15,
    );
    assert_eq!(dates.len(), 3);
    assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 9, 15).unwrap());
    assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 10, 15).unwrap());
    assert_eq!(dates[2], NaiveDate::from_ymd_opt(2024, 11, 15).unwrap());
}
#[test]
fn get_dates_between_monthly_none() {
    let dates = get_dates_between_monthly(
        &NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
        &NaiveDate::from_ymd_opt(2024, 9, 14).unwrap(),
        15,
    );
    assert_eq!(dates.len(), 0);
}

#[test]
fn get_dates_between_weekly_one() {
    let dates = get_dates_between_weekly(
        &NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
        &NaiveDate::from_ymd_opt(2024, 9, 8).unwrap(),
        3,
    );
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 9, 4).unwrap());
}

#[test]
fn get_dates_between_weekly_mult() {
    let dates = get_dates_between_weekly(
        &NaiveDate::from_ymd_opt(2024, 9, 15).unwrap(),
        &NaiveDate::from_ymd_opt(2024, 10, 15).unwrap(),
        3,
    );
    assert_eq!(dates.len(), 4);
    assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 9, 18).unwrap());
    assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 9, 25).unwrap());
    assert_eq!(dates[2], NaiveDate::from_ymd_opt(2024, 10, 2).unwrap());
    assert_eq!(dates[3], NaiveDate::from_ymd_opt(2024, 10, 9).unwrap());
}
#[test]
fn get_dates_between_weekly_none() {
    let dates = get_dates_between_weekly(
        &NaiveDate::from_ymd_opt(2024, 9, 1).unwrap(),
        &NaiveDate::from_ymd_opt(2024, 9, 3).unwrap(),
        3,
    );
    assert_eq!(dates.len(), 0);
}
