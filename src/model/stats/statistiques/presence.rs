use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Decode, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub(crate) struct Presences {
    pub(crate) values: Vec<Presence>,
}

impl Presences {
    pub(crate) fn get_max_date_range(&self) -> (String, String) {
        let start = self
            .values
            .first()
            .unwrap_or(&Presence::default())
            .date
            .clone();
        let end = self
            .values
            .last()
            .unwrap_or(&Presence::default())
            .date
            .clone();
        (start, end)
    }
}
impl Filterable<Presence> for Presences {
    fn from_vec(values: Vec<Presence>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Total".to_string(),
            "Actif".to_string(),
            "Visites".to_string(),
        ]
    }

    fn get_data(&self) -> Vec<DateData> {
        self.values
            .iter()
            .map(|x| DateData {
                date: x.get_date(),
                values: x.get_values(),
            })
            .collect()
    }

    fn get_total(&self) -> Vec<u32> {
        let total = self
            .values
            .iter()
            .fold(Presence::default(), |acc, x| Presence {
                date: x.date.clone(),
                total: acc.total + x.total,
                active: acc.active + x.active,
                visits: acc.visits + x.visits,
            });
        vec![total.total, total.active, total.visits]
    }

    fn get_max(&self) -> (usize, u32) {
        let mut max = 0;
        let mut index = 0;
        for (i, value) in self.get_data().iter().enumerate() {
            let val = value.values.first().unwrap_or(&0);
            if *val > max {
                max = *val;
                index = i;
            }
        }
        (index, max)
    }

    fn get_greatest(&self) -> (String, u32) {
        let (index, max) = self.get_max();
        let date = self
            .values
            .get(index)
            .unwrap_or(&Presence::default())
            .date
            .clone();
        (date, max)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Presence {
    pub(crate) date: String,
    pub(crate) total: u32,
    pub(crate) active: u32,
    pub(crate) visits: u32,
}

impl Display for Presence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Presence")
    }
}
impl Stat for Presence {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![self.total, self.active, self.visits]
    }

    fn get_max_value(&self) -> (usize, u32) {
        let mut max = 0;
        let mut index = 0;
        for (i, value) in self.get_values().iter().enumerate() {
            if *value > max {
                max = *value;
                index = i;
            }
        }
        (index, max)
    }

    fn get_row(&self) -> Vec<String> {
        vec![
            self.date.to_string(),
            self.total.to_string(),
            self.active.to_string(),
            self.visits.to_string(),
        ]
    }
}
