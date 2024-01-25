use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Employment {
    pub(crate) date: String,
    pub(crate) unemployed: u32,
    pub(crate) employed: u32,
}

impl Display for Employment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Emploi")
    }
}
impl Stat for Employment {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }

    fn get_values(&self) -> Vec<u32> {
        vec![self.unemployed, self.employed]
    }

    fn get_max_value(&self) -> (usize, u32) {
        if self.unemployed > self.employed {
            (0, self.unemployed)
        } else {
            (1, self.employed)
        }
    }

    fn get_row(&self) -> Vec<String> {
        vec![
            self.date.to_string(),
            self.unemployed.to_string(),
            self.employed.to_string(),
        ]
    }
}

#[derive(Decode, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub(crate) struct Employments {
    pub(crate) values: Vec<Employment>,
}

impl Filterable<Employment> for Employments {
    fn from_vec(values: Vec<Employment>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec!["Employés".to_string(), "Sans emploi".to_string()]
    }

    fn get_data(&self) -> Vec<DateData> {
        let mut data = Vec::with_capacity(self.values.len());
        for value in self.values.iter() {
            data.push(DateData {
                date: value.get_date(),
                values: vec![value.unemployed, value.employed],
            });
        }
        data
    }


    fn get_total(&self) -> Vec<u32> {
        let mut total = vec![0; 2];
        for value in self.values.iter() {
            total[0] += value.unemployed;
            total[1] += value.employed;
        }
        total
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
        let default = String::from("Sans emploi");
        let (index, max) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), max)
    }
}
