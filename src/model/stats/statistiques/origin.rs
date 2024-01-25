#![allow(non_snake_case)]

use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Origin {
    pub(crate) date: String,
    pub(crate) northAmerican: u32,
    pub(crate) southAmerican: u32,
    pub(crate) centralAmerican: u32,
    pub(crate) asian: u32,
    pub(crate) african: u32,
    pub(crate) european: u32,
    pub(crate) other: u32,
}
impl Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Origin")
    }
}
impl Stat for Origin {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.northAmerican,
            self.southAmerican,
            self.centralAmerican,
            self.asian,
            self.african,
            self.european,
            self.other,
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
            self.northAmerican.to_string(),
            self.southAmerican.to_string(),
            self.centralAmerican.to_string(),
            self.asian.to_string(),
            self.african.to_string(),
            self.european.to_string(),
            self.other.to_string(),
        ]
    }
}

#[derive(Debug, Decode, Serialize, Deserialize, Default, Clone, PartialEq)]
pub(crate) struct Origins {
    pub(crate) values: Vec<Origin>,
}

impl Origins {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec![
            "NA".to_string(),
            "SA".to_string(),
            "CA".to_string(),
            "AS".to_string(),
            "AF".to_string(),
            "EU".to_string(),
            "OT".to_string(),
        ]
    }
}

impl Filterable<Origin> for Origins {
    fn from_vec(values: Vec<Origin>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Amérique du Nord".to_string(),
            "Amérique du Sud".to_string(),
            "Amérique central".to_string(),
            "Asie".to_string(),
            "Afrique".to_string(),
            "Europe".to_string(),
            "Autre".to_string(),
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
        self.values.iter().fold(vec![0; 7], |acc, x| {
            acc.iter().zip(x.get_values()).map(|(a, b)| a + b).collect()
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
        let default = String::from("Autre");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
