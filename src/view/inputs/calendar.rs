use crate::model::beneficiaries::details::Presence;
use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;

static WEEKDAYS: [&str; 7] = ["dim.", "lun.", "mar.", "mer.", "jeu.", "ven.", "sam."];

#[derive(Clone, Debug, Props, PartialEq)]
pub struct Day {
    pub day: u32,
    pub is_today: bool,
    pub is_present: bool,
    pub is_not_current_month: bool,
}

impl Day {
    pub fn new(day: u32, is_today: bool, is_present: bool, is_not_current_month: bool) -> Self {
        Day {
            day,
            is_today,
            is_present,
            is_not_current_month,
        }
    }
}

#[derive(Clone, Debug, Props, PartialEq)]
pub struct Month {
    pub weeks: Vec<Vec<Day>>,
}

impl Month {
    pub fn new() -> Self {
        Month {
            weeks: vec![vec![], vec![], vec![], vec![], vec![], vec![]],
        }
    }
    pub fn push(&mut self, day: Day) {
        for week in self.weeks.iter_mut() {
            if week.len() == 7 {
                continue;
            }
            week.push(day);
            return;
        }
    }
}

#[derive(Props)]
pub struct Calendar<'a> {
    pub on_click: EventHandler<'a, &'a Day>,
    pub on_dbclick: EventHandler<'a, &'a Day>,
    pub current_month: u32,
    pub current_year: i32,
    pub current_day: u32,
    pub month: Month,
}

pub fn get_weeks(current_year: i32, current_month: u32, presences: &[Presence]) -> Month {
    let day = 1;
    let current_date = NaiveDate::from_ymd_opt(current_year, current_month, day).expect("No date");
    let first_day = current_date.weekday().number_from_sunday();

    let last_year = if current_month == 1 {
        current_year - 1
    } else {
        current_year
    };
    let last_month = if current_month == 1 {
        12
    } else {
        current_month - 1
    };
    let last_date = NaiveDate::from_ymd_opt(last_year, last_month, 1).unwrap();
    let last_month_days = last_date
        .signed_duration_since(current_date)
        .num_days()
        .abs();

    let next_year = if current_month == 12 {
        current_year + 1
    } else {
        current_year
    };
    let next_month = if current_month == 12 {
        1
    } else {
        current_month + 1
    };
    let next_date = NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap();
    let days_in_month = current_date
        .signed_duration_since(next_date)
        .num_days()
        .abs();

    let mut month: Month = Month::new();

    let mut count = 0;
    let mut month_days = 1;
    let mut day = 1;
    let today = Local::now().day();

    let present_days = presences
        .iter()
        .filter_map(|presence| {
            let date = NaiveDate::parse_from_str(presence.Date.as_str(), "%Y-%m-%d").unwrap();
            if date.month() == current_date.month() && date.year() == current_date.year() {
                Some(date.day())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    while count <= 44 {
        count += 1;
        if count < first_day {
            let day = last_month_days.unsigned_abs() as u32 - first_day + count;
            month.push(Day::new(day, false, false, true));
            continue;
        }
        if month_days <= days_in_month {
            let is_present = present_days.contains(&(month_days as u32));
            let is_today = today == month_days as u32
                && current_month == Local::now().month()
                && current_year == Local::now().year();
            month.push(Day::new(month_days as u32, is_today, is_present, false));
            month_days += 1;
            continue;
        }
        month.push(Day::new(day, false, false, true));
        day += 1;
    }
    month
}

#[component]
pub fn CalendarInputMut<'a>(cx: &'a Scoped<'a, Calendar<'a>>) -> Element<'a> {
    render! {
            table{
                thead{
                    tr{
                        for weekday in WEEKDAYS.iter(){
                            th{
                                "{weekday}",
                            }
                        }
                    }
                },
                tbody{
                    for week in cx.props.month.weeks.iter(){
                        tr{
                            for day in week.iter(){
                               td{
                                    onclick: |_| { cx.props.on_click.call(day) },
                                    ondblclick: |_| { cx.props.on_dbclick.call(day)},
                                    class: "{get_class(day)}",
                                    "{format_date(day)}",
                               }
                            }
                        }
                    }
                }
            }
    }
}

fn format_date(day: &Day) -> String {
    if day.day < 10 {
        format!("0{}", day.day)
    } else {
        day.day.to_string()
    }
}

fn get_class(day: &Day) -> String {
    let mut class = String::from("calendar-day ");
    if day.is_today {
        class.push_str("today ");
    }
    if day.is_present {
        class.push_str("present ");
    }
    if day.is_not_current_month {
        class.push_str("not-current-month ");
    }
    class
}
