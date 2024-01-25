use crate::model::stats::stats::DateData;
use chrono::NaiveDate;
use std::fmt::Debug;

pub(crate) trait Stat
where
    Self: Debug,
{
    fn get_date(&self) -> NaiveDate;
    fn get_values(&self) -> Vec<u32>;
    fn get_max_value(&self) -> (usize, u32);
    fn get_row(&self) -> Vec<String>;
}

pub(crate) trait Filterable<Y>
where
    Self: Sized,
    Y: Stat,
{
    fn from_vec(vec: Vec<Y>) -> Self;
    fn get_labels() -> Vec<String>;
    fn get_data(&self) -> Vec<DateData>;
    fn get_total(&self) -> Vec<u32>;
    fn get_max(&self) -> (usize, u32);
    fn get_greatest(&self) -> (String, u32);
}
