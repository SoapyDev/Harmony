#![allow(non_snake_case)]

use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct City {
    pub(crate) date: String,
    pub(crate) carignan: u32,
    pub(crate) chambly: u32,
    pub(crate) marieville: u32,
    pub(crate) richelieu: u32,
    pub(crate) stMathias: u32,
    pub(crate) other: u32,
}

impl Display for City {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ville")
    }
}
impl Stat for City {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.carignan,
            self.chambly,
            self.marieville,
            self.richelieu,
            self.stMathias,
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
            self.carignan.to_string(),
            self.chambly.to_string(),
            self.marieville.to_string(),
            self.richelieu.to_string(),
            self.stMathias.to_string(),
            self.other.to_string(),
        ]
    }
}

#[derive(Decode, Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct Cities {
    pub(crate) values: Vec<City>,
}

impl Cities {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec![
            "CA".to_string(),
            "CH".to_string(),
            "MA".to_string(),
            "RI".to_string(),
            "SM".to_string(),
            "OT".to_string(),
        ]
    }
}

impl Filterable<City> for Cities {
    fn from_vec(values: Vec<City>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Carignan".to_string(),
            "Chambly".to_string(),
            "Marieville".to_string(),
            "Richelieu".to_string(),
            "St-Mathias".to_string(),
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
        self.values
            .iter()
            .fold(City::default(), |acc, x| City {
                date: x.date.clone(),
                carignan: acc.carignan + x.carignan,
                chambly: acc.chambly + x.chambly,
                marieville: acc.marieville + x.marieville,
                richelieu: acc.richelieu + x.richelieu,
                stMathias: acc.stMathias + x.stMathias,
                other: acc.other + x.other,
            })
            .get_values()
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
        let default = String::from("Carignan");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
