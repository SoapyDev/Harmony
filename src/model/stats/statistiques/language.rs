use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Language {
    pub(crate) date: String,
    pub(crate) french: u32,
    pub(crate) english: u32,
    pub(crate) spanish: u32,
    pub(crate) arabic: u32,
    pub(crate) mandarin: u32,
    pub(crate) other: u32,
}
impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Langue")
    }
}
impl Stat for Language {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.french,
            self.english,
            self.spanish,
            self.arabic,
            self.mandarin,
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
            self.french.to_string(),
            self.english.to_string(),
            self.spanish.to_string(),
            self.arabic.to_string(),
            self.mandarin.to_string(),
            self.other.to_string(),
        ]
    }
}
#[derive(Decode, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct Languages {
    pub(crate) values: Vec<Language>,
}

impl Languages {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec![
            "FR".to_string(),
            "EN".to_string(),
            "ES".to_string(),
            "AR".to_string(),
            "CH".to_string(),
            "OT".to_string(),
        ]
    }
}
impl Filterable<Language> for Languages {
    fn from_vec(values: Vec<Language>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "Français".to_string(),
            "Anglais".to_string(),
            "Espagnol".to_string(),
            "Arabe".to_string(),
            "Mandarin".to_string(),
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
        let default = String::from("Français");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
