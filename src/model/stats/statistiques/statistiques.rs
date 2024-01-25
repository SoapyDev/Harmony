use crate::model::stats::stats::{DateData, Stats};
use crate::model::stats::traits::Filterable;
use dioxus_hooks::UseSharedState;

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) struct Statistiques {
    pub(crate) total: u32,
    pub(crate) active: u32,
    pub(crate) visits: u32,
    pub(crate) age: String,
    pub(crate) sexe: String,
    pub(crate) language: String,
    pub(crate) origin: String,
    pub(crate) kid: String,
    pub(crate) family_situation: String,
    pub(crate) city: String,
    pub(crate) study: String,
    pub(crate) employment: String,
    pub(crate) income: String,
}

impl From<&Stats> for Statistiques {
    fn from(value: &Stats) -> Self {
        let total = value
            .presences
            .get_data()
            .last()
            .unwrap_or(&DateData::default())
            .values
            .clone();
        let age = value.ages.get_greatest();
        let sexe = value.sexes.get_greatest();
        let language = value.languages.get_greatest();
        let origin = value.origins.get_greatest();
        let kid = value.kids.get_greatest();
        let family_situation = value.familySituations.get_greatest();
        let city = value.cities.get_greatest();
        let study = value.studies.get_greatest();
        let employment = value.employments.get_greatest();
        let income = value.incomes.get_greatest();

        Self {
            total: total[0],
            active: total[1],
            visits: total[2],
            age: age.0,
            sexe: sexe.0,
            language: language.0,
            origin: origin.0,
            kid: kid.0,
            family_situation: family_situation.0,
            city: city.0,
            study: study.0,
            employment: employment.0,
            income: income.0,
        }
    }
}
impl From<UseSharedState<Stats>> for Statistiques {
    fn from(stats: UseSharedState<Stats>) -> Self {
        let total = stats
            .read()
            .presences
            .get_data()
            .last()
            .unwrap_or(&DateData::default())
            .values
            .clone();
        let age = stats.read().ages.get_greatest();
        let sexe = stats.read().sexes.get_greatest();
        let language = stats.read().languages.get_greatest();
        let origin = stats.read().origins.get_greatest();
        let kid = stats.read().kids.get_greatest();
        let family_situation = stats.read().familySituations.get_greatest();
        let city = stats.read().cities.get_greatest();
        let study = stats.read().studies.get_greatest();
        let employment = stats.read().employments.get_greatest();
        let income = stats.read().incomes.get_greatest();

        Self {
            total: total[0],
            active: total[1],
            visits: total[2],
            age: age.0,
            sexe: sexe.0,
            language: language.0,
            origin: origin.0,
            kid: kid.0,
            family_situation: family_situation.0,
            city: city.0,
            study: study.0,
            employment: employment.0,
            income: income.0,
        }
    }
}
