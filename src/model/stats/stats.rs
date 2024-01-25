#![allow(non_snake_case)]

use crate::controler::connection::Token;
use crate::model::stats::statistiques::age::{Age, Ages};
use crate::model::stats::statistiques::city::{Cities, City};
use crate::model::stats::statistiques::employment::{Employment, Employments};
use crate::model::stats::statistiques::family_situation::{FamilySituation, FamilySituations};
use crate::model::stats::statistiques::income::{Income, Incomes};
use crate::model::stats::statistiques::kid::{Kid, Kids};
use crate::model::stats::statistiques::language::{Language, Languages};
use crate::model::stats::statistiques::origin::{Origin, Origins};
use crate::model::stats::statistiques::presence::{Presence, Presences};
use crate::model::stats::statistiques::sexe::{Sexe, Sexes};
use crate::model::stats::statistiques::study::{Studies, Study};
use crate::model::stats::traits::Filterable;
use crate::model::users::user::User;
use bincode::Decode;
use chrono::NaiveDate;
use dioxus_hooks::UseSharedState;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Decode, Serialize, Deserialize, Debug)]
pub(crate) struct StatsRes {
    pub(crate) Presences: Vec<Presence>,
    pub(crate) Ages: Vec<Age>,
    pub(crate) Cities: Vec<City>,
    pub(crate) Employments: Vec<Employment>,
    pub(crate) FamilySituations: Vec<FamilySituation>,
    pub(crate) Incomes: Vec<Income>,
    pub(crate) Kids: Vec<Kid>,
    pub(crate) Languages: Vec<Language>,
    pub(crate) Origins: Vec<Origin>,
    pub(crate) Sexes: Vec<Sexe>,
    pub(crate) Studies: Vec<Study>,
}

#[derive(Decode, Serialize, Deserialize, Debug)]
pub(crate) struct Stats {
    pub(crate) presences: Presences,
    pub(crate) ages: Ages,
    pub(crate) cities: Cities,
    pub(crate) employments: Employments,
    pub(crate) familySituations: FamilySituations,
    pub(crate) incomes: Incomes,
    pub(crate) kids: Kids,
    pub(crate) languages: Languages,
    pub(crate) origins: Origins,
    pub(crate) sexes: Sexes,
    pub(crate) studies: Studies,
}

impl Stats {
    pub(crate) fn new() -> Self {
        Self {
            presences: Presences::default(),
            ages: Ages::default(),
            cities: Cities::default(),
            employments: Employments::default(),
            familySituations: FamilySituations::default(),
            incomes: Incomes::default(),
            kids: Kids::default(),
            languages: Languages::default(),
            origins: Origins::default(),
            sexes: Sexes::default(),
            studies: Studies::default(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.presences.values.is_empty()
    }

    pub(crate) async fn fetch(user: UseSharedState<User>) -> Stats {
        let token = Token::new(user.read().session.clone());
        Self::get_stats(token).await.unwrap_or(Stats::new())
    }

    pub(crate) fn get_labels() -> Vec<String> {
        vec![
            "Présence".to_string(),
            "Âge".to_string(),
            "Ville".to_string(),
            "Emploi".to_string(),
            "Situation familiale".to_string(),
            "Revenu".to_string(),
            "Enfant".to_string(),
            "Langue".to_string(),
            "Origine".to_string(),
            "Sexe".to_string(),
            "Étude".to_string(),
        ]
    }
}

impl From<StatsRes> for Stats {
    fn from(value: StatsRes) -> Self {
        Self {
            presences: Presences::from_vec(value.Presences),
            ages: Ages::from_vec(value.Ages),
            cities: Cities::from_vec(value.Cities),
            employments: Employments::from_vec(value.Employments),
            familySituations: FamilySituations::from_vec(value.FamilySituations),
            incomes: Incomes::from_vec(value.Incomes),
            kids: Kids::from_vec(value.Kids),
            languages: Languages::from_vec(value.Languages),
            origins: Origins::from_vec(value.Origins),
            sexes: Sexes::from_vec(value.Sexes),
            studies: Studies::from_vec(value.Studies),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) enum StatType {
    Presence(Presences),
    Age(Ages),
    City(Cities),
    Employment(Employments),
    FamilySituation(FamilySituations),
    Income(Incomes),
    Kid(Kids),
    Language(Languages),
    Origin(Origins),
    Sexe(Sexes),
    Study(Studies),
}

impl StatType {
    pub(crate) fn new(stats: &UseSharedState<Stats>, value: &str) -> Self {
        match value {
            "Présence" => StatType::Presence(stats.read().presences.clone()),
            "Âge" => StatType::Age(stats.read().ages.clone()),
            "Ville" => StatType::City(stats.read().cities.clone()),
            "Emploi" => StatType::Employment(stats.read().employments.clone()),
            "Situation familiale" => {
                StatType::FamilySituation(stats.read().familySituations.clone())
            }
            "Revenu" => StatType::Income(stats.read().incomes.clone()),
            "Enfant" => StatType::Kid(stats.read().kids.clone()),
            "Langue" => StatType::Language(stats.read().languages.clone()),
            "Origine" => StatType::Origin(stats.read().origins.clone()),
            "Sexe" => StatType::Sexe(stats.read().sexes.clone()),
            "Étude" => StatType::Study(stats.read().studies.clone()),
            _ => StatType::Presence(stats.read().presences.clone()),
        }
    }

    pub(crate) fn get_value(&self) -> String {
        match self {
            StatType::Presence(_) => "Présence".to_string(),
            StatType::Age(_) => "Âge".to_string(),
            StatType::City(_) => "Ville".to_string(),
            StatType::Employment(_) => "Emploi".to_string(),
            StatType::FamilySituation(_) => "Situation familiale".to_string(),
            StatType::Income(_) => "Revenu".to_string(),
            StatType::Kid(_) => "Enfant".to_string(),
            StatType::Language(_) => "Langue".to_string(),
            StatType::Origin(_) => "Origine".to_string(),
            StatType::Sexe(_) => "Sexe".to_string(),
            StatType::Study(_) => "Étude".to_string(),
        }
    }
    pub(crate) fn get_type(&self) -> String {
        match self {
            StatType::Presence(_) => "presence".to_string(),
            StatType::Age(_) => "âge".to_string(),
            StatType::City(_) => "ville".to_string(),
            StatType::Employment(_) => "situation d'emploi".to_string(),
            StatType::FamilySituation(_) => "situation familial".to_string(),
            StatType::Income(_) => "revenu".to_string(),
            StatType::Kid(_) => "enfants".to_string(),
            StatType::Language(_) => "language".to_string(),
            StatType::Origin(_) => "origine".to_string(),
            StatType::Sexe(_) => "sexe".to_string(),
            StatType::Study(_) => "niveau d'étude".to_string(),
        }
    }
    pub(crate) fn get_labels(&self) -> Vec<String> {
        match self {
            StatType::Presence(_) => Presences::get_labels(),
            StatType::Age(_) => Ages::get_labels(),
            StatType::City(_) => Cities::get_labels(),
            StatType::Employment(_) => Employments::get_labels(),
            StatType::FamilySituation(_) => FamilySituations::get_labels(),
            StatType::Income(_) => Incomes::get_labels(),
            StatType::Kid(_) => Kids::get_labels(),
            StatType::Language(_) => Languages::get_labels(),
            StatType::Origin(_) => Origins::get_labels(),
            StatType::Sexe(_) => Sexes::get_labels(),
            StatType::Study(_) => Studies::get_labels(),
        }
    }

    pub(crate) fn get_max(&self) -> (usize, u32) {
        match self {
            StatType::Presence(p) => p.get_max(),
            StatType::Age(a) => a.get_max(),
            StatType::City(c) => c.get_max(),
            StatType::Employment(e) => e.get_max(),
            StatType::FamilySituation(f) => f.get_max(),
            StatType::Income(i) => i.get_max(),
            StatType::Kid(k) => k.get_max(),
            StatType::Language(l) => l.get_max(),
            StatType::Origin(o) => o.get_max(),
            StatType::Sexe(s) => s.get_max(),
            StatType::Study(s) => s.get_max(),
        }
    }

    pub(crate) fn get_data(&self) -> Vec<DateData> {
        match self {
            StatType::Presence(p) => p.get_data(),
            StatType::Age(a) => a.get_data(),
            StatType::City(c) => c.get_data(),
            StatType::Employment(e) => e.get_data(),
            StatType::FamilySituation(f) => f.get_data(),
            StatType::Income(i) => i.get_data(),
            StatType::Kid(k) => k.get_data(),
            StatType::Language(l) => l.get_data(),
            StatType::Origin(o) => o.get_data(),
            StatType::Sexe(s) => s.get_data(),
            StatType::Study(s) => s.get_data(),
        }
    }
}

#[derive(Clone, PartialEq, Default)]
pub(crate) struct DateData {
    pub(crate) date: NaiveDate,
    pub(crate) values: Vec<u32>,
}
