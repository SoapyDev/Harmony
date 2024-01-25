#![allow(non_snake_case)]

use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Study {
    pub(crate) date: String,
    pub(crate) noStudy: u32,
    pub(crate) primarySchool: u32,
    pub(crate) highSchool: u32,
    pub(crate) college: u32,
    pub(crate) university: u32,
    pub(crate) other: u32,
}

impl Display for Study {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Study")
    }
}
impl Stat for Study {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![
            self.noStudy,
            self.primarySchool,
            self.highSchool,
            self.college,
            self.university,
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
            self.noStudy.to_string(),
            self.primarySchool.to_string(),
            self.highSchool.to_string(),
            self.college.to_string(),
            self.university.to_string(),
            self.other.to_string(),
        ]
    }
}

#[derive(Decode, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct Studies {
    pub(crate) values: Vec<Study>,
}

impl Studies {
    pub(crate) fn get_selectable_values() -> Vec<String> {
        vec![
            String::from("NS"),
            String::from("EL"),
            String::from("HS"),
            String::from("CP"),
            String::from("UN"),
            String::from("OT"),
        ]
    }
}
impl Filterable<Study> for Studies {
    fn from_vec(values: Vec<Study>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            String::from("Aucune étude"),
            String::from("Primaire"),
            String::from("Secondaire"),
            String::from("Collège"),
            String::from("Université"),
            String::from("Autre"),
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
            let values = x.get_values();
            acc.iter().zip(values.iter()).map(|(a, b)| a + b).collect()
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
        let default = String::from("Aucune étude");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
