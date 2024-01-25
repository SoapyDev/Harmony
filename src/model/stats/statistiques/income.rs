#![allow(non_snake_case)]
use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Income {
    pub(crate) date: String,
    pub(crate) noIncome: u32,
    pub(crate) income_1_14999: u32,
    pub(crate) income_15000_29999: u32,
    pub(crate) income_30000_more: u32,
}

impl Display for Income {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Salaire")
    }
}

impl Stat for Income {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.noIncome,
            self.income_1_14999,
            self.income_15000_29999,
            self.income_30000_more,
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
            self.noIncome.to_string(),
            self.income_1_14999.to_string(),
            self.income_15000_29999.to_string(),
            self.income_30000_more.to_string(),
        ]
    }
}

#[derive(Decode, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct Incomes {
    pub(crate) values: Vec<Income>,
}

impl Incomes {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec![
            "N".to_string(),
            "L".to_string(),
            "M".to_string(),
            "H".to_string(),
        ]
    }
}
impl Filterable<Income> for Incomes {
    fn from_vec(values: Vec<Income>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Pas de revenus".to_string(),
            "1$ à 14,999$".to_string(),
            "15,000$ à 29,999$".to_string(),
            "30,000$ et plus".to_string(),
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
                    acc[0] + x.noIncome,
                    acc[1] + x.income_1_14999,
                    acc[2] + x.income_15000_29999,
                    acc[3] + x.income_30000_more,
                ]
            })
    }
    fn get_max(&self) -> (usize, u32) {
        self.values.iter().fold((0, 0), |acc, x| {
            let max = x.get_max_value();
            if max.1 > acc.1 {
                max
            } else {
                acc
            }
        })
    }

    fn get_greatest(&self) -> (String, u32) {
        let default = String::from("N/A");
        let (index, max) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), max)
    }
}
