use crate::model::stats::stats::DateData;
use chrono::{Datelike, NaiveDate};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct DateRange {
    from: NaiveDate,
    to: NaiveDate,
}

impl Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.from.year() == self.to.year() && self.from.month() == self.to.month() {
            return write!(
                f,
                "{}_{}",
                self.from.format("%d"),
                self.to.format("%d %b %Y")
            );
        } else if self.from.year() == self.to.year() {
            return write!(
                f,
                "{}_{}",
                self.from.format("%d-%b"),
                self.to.format("%d-%b %Y")
            );
        }
        write!(
            f,
            "{}_{}",
            self.from.format("%d-%b-%y"),
            self.to.format("%d-%b-%y")
        )
    }
}
impl DateRange {
    pub(crate) fn new(from: &str, to: &str) -> Self {
        let from = NaiveDate::parse_from_str(from, "%Y-%m-%d").unwrap_or_default();
        let to = NaiveDate::parse_from_str(to, "%Y-%m-%d").unwrap_or_default();
        Self { from, to }
    }
    pub(crate) fn get_dates(&self) -> (NaiveDate, NaiveDate) {
        (self.from, self.to)
    }
    pub(crate) fn filter(&self, data: Vec<DateData>) -> Vec<DateData> {
        data.into_iter()
            .filter(|d| {
                let date = d.date;
                date >= self.from && date <= self.to
            })
            .collect()
    }

    pub(crate) fn set_from(&mut self, from: &str) {
        self.from = NaiveDate::parse_from_str(from, "%Y-%m-%d").unwrap_or_default();
    }
    pub(crate) fn get_from(&self) -> String {
        self.from.format("%Y-%m-%d").to_string()
    }
    pub(crate) fn set_to(&mut self, to: &str) {
        self.to = NaiveDate::parse_from_str(to, "%Y-%m-%d").unwrap_or_default();
    }
    pub(crate) fn get_to(&self) -> String {
        self.to.format("%Y-%m-%d").to_string()
    }
}
