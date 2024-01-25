#![allow(non_snake_case)]

use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use crate::model::stats::stats::DateData;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct FamilySituation {
    pub(crate) date: String,
    pub(crate) single: u32,
    pub(crate) couple: u32,
    pub(crate) coupleKids: u32,
    pub(crate) recomposed: u32,
    pub(crate) singleParent: u32,
    pub(crate) other: u32,
}

impl Display for FamilySituation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Situation familiale")
    }
}
impl Stat for FamilySituation {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.single,
            self.couple,
            self.coupleKids,
            self.recomposed,
            self.singleParent,
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
            self.single.to_string(),
            self.couple.to_string(),
            self.coupleKids.to_string(),
            self.recomposed.to_string(),
            self.singleParent.to_string(),
            self.other.to_string(),
        ]
    }
}
#[derive(Decode, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct FamilySituations {
    pub(crate) values: Vec<FamilySituation>,
}

impl FamilySituations {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec![
            "SG".to_string(),
            "CP".to_string(),
            "CK".to_string(),
            "RC".to_string(),
            "SP".to_string(),
            "OT".to_string(),
        ]
    }
}
impl Filterable<FamilySituation> for FamilySituations {
    fn from_vec(values: Vec<FamilySituation>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Célibataire".to_string(),
            "Couple".to_string(),
            "Couple avec enfants".to_string(),
            "Recomposée".to_string(),
            "Monoparental".to_string(),
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
        self.values.iter().fold(vec![0; 6], |acc, x| {
            acc.iter()
                .zip(x.get_values())
                .map(|(a, b)| a + b)
                .collect()
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
        let default = String::from("Célibataire");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
