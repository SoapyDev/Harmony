use crate::model::stats::stats::DateData;
use crate::model::stats::traits::{Filterable, Stat};
use bincode::Decode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Decode)]
pub(crate) struct Age {
    pub(crate) date: String,
    pub(crate) age_0_19: u32,
    pub(crate) age_20_29: u32,
    pub(crate) age_30_39: u32,
    pub(crate) age_40_49: u32,
    pub(crate) age_50_59: u32,
    pub(crate) age_60_69: u32,
    pub(crate) age_70_plus: u32,
}

impl Display for Age {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Age")
    }
}
impl Stat for Age {
    fn get_date(&self) -> NaiveDate {
        self.date.parse::<NaiveDate>().unwrap_or_default()
    }
    fn get_values(&self) -> Vec<u32> {
        vec![
            self.age_0_19,
            self.age_20_29,
            self.age_30_39,
            self.age_40_49,
            self.age_50_59,
            self.age_60_69,
            self.age_70_plus,
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
            self.age_0_19.to_string(),
            self.age_20_29.to_string(),
            self.age_30_39.to_string(),
            self.age_40_49.to_string(),
            self.age_50_59.to_string(),
            self.age_60_69.to_string(),
            self.age_70_plus.to_string(),
        ]
    }
}
#[derive(Decode, Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
pub(crate) struct Ages {
    pub(crate) values: Vec<Age>,
}

impl Filterable<Age> for Ages {
    fn from_vec(values: Vec<Age>) -> Self {
        Self { values }
    }
    fn get_labels() -> Vec<String> {
        vec![
            "0-19 ans".to_string(),
            "20-29 ans".to_string(),
            "30-39 ans".to_string(),
            "40-49 ans".to_string(),
            "50-59 ans".to_string(),
            "60-69 ans".to_string(),
            "70+ ans".to_string(),
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
                    acc[0] + x.age_0_19,
                    acc[1] + x.age_20_29,
                    acc[2] + x.age_30_39,
                    acc[3] + x.age_40_49,
                    acc[4] + x.age_50_59,
                    acc[5] + x.age_60_69,
                    acc[6] + x.age_70_plus,
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
        let default = String::from("0-19");
        let (index, value) = self.get_max();
        let labels = Self::get_labels();
        let label = labels.get(index).unwrap_or(&default);
        (label.to_string(), value)
    }
}
