use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Sexe {
    pub(crate) date: String,
    pub(crate) male: u32,
    pub(crate) female: u32,
    pub(crate) other: u32,
}

impl Display for Sexe {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sexe")
    }
}

impl Stat for Sexe {
    fn get_date(&self) -> NaiveDate {
        NaiveDate::parse_from_str(&self.date, "%Y-%m-%d").unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![self.male, self.female, self.other]
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
            self.male.to_string(),
            self.female.to_string(),
            self.other.to_string(),
        ]
    }
}

#[derive(Decode, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub(crate) struct Sexes {
    values: Vec<Sexe>,
}

impl Sexes {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec!["M".to_string(), "F".to_string(), "o".to_string()]
    }
}

impl Filterable<Sexe> for Sexes {
    fn from_vec(values: Vec<Sexe>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Homme".to_string(),
            "Femme".to_string(),
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
        self.values.iter().fold(vec![0, 0, 0], |acc, x| {
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
        let default = "Aucun".to_string();
        let (index, max) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), max)
    }
}
