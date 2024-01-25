#![allow(non_snake_case)]

use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Kid {
    pub(crate) date: String,
    pub(crate) noKids: u32,
    pub(crate) oneKid: u32,
    pub(crate) twoKids: u32,
    pub(crate) threeToFourKids: u32,
    pub(crate) fivePlusKids: u32,
}

impl Display for Kid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enfant(s)")
    }
}
impl Stat for Kid {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.noKids,
            self.oneKid,
            self.twoKids,
            self.threeToFourKids,
            self.fivePlusKids,
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
            self.noKids.to_string(),
            self.oneKid.to_string(),
            self.twoKids.to_string(),
            self.threeToFourKids.to_string(),
            self.fivePlusKids.to_string(),
        ]
    }
}

#[derive(Decode, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct Kids {
    pub(crate) values: Vec<Kid>,
}

impl Filterable<Kid> for Kids {
    fn from_vec(values: Vec<Kid>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Aucun enfants".to_string(),
            "1 enfant".to_string(),
            "2 enfants".to_string(),
            "3 à 4 enfants".to_string(),
            "5 enfants et plus".to_string(),
        ]
    }

    fn get_data(&self) -> Vec<DateData> {
        let mut data = vec![];
        for value in self.values.iter() {
            data.push(DateData {
                date: value.get_date(),
                values: value.get_values(),
            })
        }
        data
    }

    fn get_total(&self) -> Vec<u32> {
        self.values.iter().fold(vec![0; 5], |acc, x| {
            let mut total = acc.clone();
            for (i, value) in x.get_values().iter().enumerate() {
                total[i] += value;
            }
            total
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
        let default = "Aucun enfants".to_string();
        let (index, max) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), max)
    }
}
