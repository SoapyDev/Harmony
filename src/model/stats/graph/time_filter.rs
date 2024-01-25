use crate::model::stats::stats::DateData;
use chrono::{Datelike, Weekday};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum TimeFilter {
    Day,
    Week,
    Month,
    Year,
}

impl Display for TimeFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeFilter::Day => write!(f, "Jour"),
            TimeFilter::Week => write!(f, "Semaine"),
            TimeFilter::Month => write!(f, "Mois"),
            TimeFilter::Year => write!(f, "Année"),
        }
    }
}

impl FromStr for TimeFilter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Jour" => Ok(TimeFilter::Day),
            "Semaine" => Ok(TimeFilter::Week),
            "Mois" => Ok(TimeFilter::Month),
            "Année" => Ok(TimeFilter::Year),
            _ => Err(anyhow::anyhow!("Unknown time filter")),
        }
    }
}

impl TimeFilter {
    pub(crate) fn get_labels() -> Vec<String> {
        vec![
            TimeFilter::Day.to_string(),
            TimeFilter::Week.to_string(),
            TimeFilter::Month.to_string(),
            TimeFilter::Year.to_string(),
        ]
    }

    pub(crate) fn filter(&self, data: Vec<DateData>) -> Vec<DateData> {
        match self {
            TimeFilter::Day => data,
            TimeFilter::Week => self.filter_week(data),
            TimeFilter::Month => self.filter_month(data),
            TimeFilter::Year => self.filter_year(data),
        }
    }

    pub(crate) fn filter_week(&self, data: Vec<DateData>) -> Vec<DateData> {
        data.into_iter()
            .filter(|d| d.date.weekday() == Weekday::Sun)
            .collect()
    }

    pub(crate) fn filter_month(&self, data: Vec<DateData>) -> Vec<DateData> {
        data.into_iter().filter(|d| d.date.day() == 1).collect()
    }

    pub(crate) fn filter_year(&self, data: Vec<DateData>) -> Vec<DateData> {
        data.into_iter()
            .filter(|d| d.date.month() == 1 && d.date.day() == 1)
            .collect()
    }
}
