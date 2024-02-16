#![allow(non_snake_case)]

use std::fmt::{Display, Formatter};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Amount {
    pub(crate) date: String,
    pub(crate) total_weekly: u32,
    pub(crate) total_monthly: u32,
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Montants")
    }
}
impl Stat for Amount {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }
    fn get_values(&self) -> Vec<u32> {
        vec![
            self.total_weekly,
            self.total_monthly,
        ]
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
            self.total_weekly.to_string(),
            self.total_monthly.to_string(),
        ]
    }
}
#[derive(Decode, Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub(crate) struct Amounts {
    pub(crate) values: Vec<Amount>,
}


impl Filterable<Amount> for Amounts {
    fn from_vec(values: Vec<Amount>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Total hebdomadaire".to_string(),
            "Total mensuel".to_string(),
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
        self.values
            .iter()
            .fold(vec![0; Self::get_labels().len()], |acc, x| {
                vec![
                    acc[0] + x.total_weekly,
                    acc[1] + x.total_monthly,
                ]
            })
    }
    fn get_max(&self) -> (usize, u32) {
        self.values.iter().fold((0, 0), |acc, x| {
            let (index, max) = x.get_max_value();
            if max > acc.1 {
                (index, max)
            } else {
                acc
            }
        })
    }
    fn get_greatest(&self) -> (String, u32) {
        let default = String::from("Total hebdomadaire");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
