#![allow(non_snake_case)]

use crate::controler::connection::{
    encrypt, Token, TokenBeneficiary, TokenBeneficiaryId, TokenSearch,
};
use crate::model::beneficiaries::details::Detail;
use crate::model::users::user::User;
use bincode::Decode;
use chrono::{Datelike, Local, NaiveDate};
use dioxus_hooks::UseSharedState;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Default, Debug)]
pub struct Beneficiaries {
    pub(crate) beneficiaries: Vec<Beneficiary>,
}

impl From<Vec<Beneficiary>> for Beneficiaries {
    fn from(value: Vec<Beneficiary>) -> Self {
        Self {
            beneficiaries: value,
        }
    }
}

impl From<Vec<&Beneficiary>> for Beneficiaries {
    fn from(value: Vec<&Beneficiary>) -> Self {
        Self {
            beneficiaries: value.into_iter().cloned().collect::<Vec<Beneficiary>>(),
        }
    }
}

impl Beneficiaries {
    pub fn new() -> Self {
        Self {
            beneficiaries: vec![],
        }
    }
    pub fn filter(&self, filter: &str) -> Self {
        if filter.is_empty() {
            return self.clone();
        }
        let filter = filter.to_lowercase();
        let benes = self
            .beneficiaries
            .iter()
            .filter(|b| b.get_full_name().to_lowercase().contains(&filter))
            .collect::<Vec<&Beneficiary>>();
        Self::from(benes)
    }
    pub async fn get_beneficiaries(user: UseSharedState<User>) -> Beneficiaries {
        let token = Token::from(user);
        let mut benes = Self::get_all_beneficiary(token).await.unwrap_or(vec![]);
        benes.iter_mut().for_each(|b| b.decrypt());
        Beneficiaries::from(benes)
    }

    pub fn find(&self, id: &i32) -> Option<&Beneficiary> {
        self.beneficiaries.iter().find(|b| b.Id == *id)
    }

    pub fn update(&mut self, beneficiary: &Beneficiary) {
        self.beneficiaries.iter_mut().for_each(|b| {
            if b.Id == beneficiary.Id {
                *b = beneficiary.clone();
            }
        });
    }
    pub async fn search_beneficiaries(user: UseSharedState<User>, search: &str) -> Beneficiaries {
        let token_search = TokenSearch {
            Token: Token::from(user).Token,
            Search: search.to_string(),
        };
        let mut benes = Self::search(token_search).await.unwrap_or(vec![]);
        benes.iter_mut().for_each(|b| b.decrypt());
        Beneficiaries::from(benes)
    }

    pub fn insert_beneficiaries(&mut self, beneficiaries: Beneficiaries) {
        beneficiaries.beneficiaries.iter().for_each(|b| {
            if self.find(&b.Id).is_none() {
                self.beneficiaries.push(b.clone());
            }
        })
    }
}
#[derive(Clone, PartialEq, Default, Debug, Serialize, Deserialize, Decode)]
pub struct Beneficiary {
    pub(crate) Id: i32,
    pub(crate) FirstName: String,
    pub(crate) LastName: String,
    pub(crate) Email: String,
    pub(crate) Phone: String,
    pub(crate) Address: String,
    pub(crate) PostalCode: String,
    pub(crate) Kid: u8,
    pub(crate) Adult: u8,
    pub(crate) Birth: String,
    pub(crate) LastPresence: String,
    pub(crate) Sexe: String,
    pub(crate) Language: String,
    pub(crate) Origin: String,
    pub(crate) City: String,
    pub(crate) Study: String,
    pub(crate) Income: String,
    pub(crate) FamilySituation: String,
    pub(crate) IsActive: bool,
    pub(crate) IsSdf: bool,
    pub(crate) IsEmployed: bool,
    pub(crate) HasAllergies: bool,
    pub(crate) HasGeneralNote: bool,
    pub(crate) GeneralNote: String,
    pub(crate) AdminNote: String,
    pub(crate) TsNote: String,
    pub(crate) Situation: String,
}

impl Beneficiary {
    pub async fn new(user: UseSharedState<User>) -> Result<Self, String> {
        let token = Token::from(user);
        let beneficiary = Self::create(token).await;

        match beneficiary.is_err() {
            true => Err("Impossible de créer un bénéficiaire".to_string()),
            false => {
                let mut bene = beneficiary.unwrap();
                bene.decrypt();
                Ok(bene)
            }
        }
    }
    pub async fn update(&self, user: UseSharedState<User>) -> bool {
        let token_bene = TokenBeneficiary::new(Token::from(user), self.encrypt());
        if Self::update_beneficiary(token_bene).await.is_err() {
            return false;
        }
        true
    }
    pub async fn get_beneficiary(user: UseSharedState<User>, id: i32) -> (Beneficiary, Detail) {
        let token_id: TokenBeneficiaryId = TokenBeneficiaryId::new(Token::from(user), id);
        let (mut bene, details) = Self::get_beneficiary_details(token_id)
            .await
            .unwrap_or((Self::default(), Detail::default()));
        bene.decrypt();
        (bene, details)
    }

    pub fn set_phone(&mut self) {
        let phone_number = Self::format_phone(self.Phone.clone());
        self.Phone = phone_number;
    }

    pub fn format_phone(phone: String) -> String {
        let validator = regex::Regex::new(r"^(\d{3}) \d{3}-\d{4}$").unwrap();
        if validator.is_match(&phone) {
            return phone;
        }

        let mut result = String::new();
        for c in phone.chars() {
            if c.is_numeric() {
                result.push(c);
            }
        }
        let phone = result.chars().take(10).collect::<Vec<char>>();
        let mut phone_number = String::new();

        for (i, c) in phone.iter().enumerate() {
            if i == 0 {
                phone_number.insert(0, '(');
            }
            if i == 3 {
                phone_number.insert(4, ')');
                phone_number.insert(5, ' ');
            }
            if i == 6 {
                phone_number.insert(9, '-');
            }
            phone_number.push(*c);
        }

        result
    }

    pub fn get_phone(&self) -> String {
        let phone = self.Phone.clone();
        Self::format_phone(phone)
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.FirstName, self.LastName)
    }
    pub fn set_postal_code(&mut self, value: &str) {
        let mut postal_code = value.to_uppercase();
        postal_code = postal_code
            .replace(' ', "")
            .chars()
            .take(6)
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>();
        if postal_code.len() > 3 {
            postal_code.insert(3, ' ');
        }

        self.PostalCode = postal_code;
    }
    pub fn get_birth(&self) -> String {
        self.Birth.to_string()
    }
    pub fn set_birth(&mut self, birth: &str) {
        self.Birth = birth.to_string();
    }

    pub fn set_last_presence(&mut self, last_presence: String) {
        self.LastPresence = last_presence.to_string();
        let now = NaiveDate::from_ymd_opt(
            Local::now().year(),
            Local::now().month(),
            Local::now().day(),
        )
        .unwrap_or_default();
        let presence = NaiveDate::from_str(&last_presence).unwrap_or_default();

        // Start of year
        let cutoff_year = NaiveDate::from_ymd_opt(now.year(), 1, 1).unwrap_or_default();

        // last 6 months
        let cutoff_month = now - chrono::Duration::days(180);

        self.IsActive = presence >= cutoff_year || presence >= cutoff_month;
    }
    pub fn set_has_general_note(&mut self) {
        self.HasGeneralNote = !self.GeneralNote.is_empty();
    }

    pub fn set_general_note(&mut self, general_note: &str) {
        self.GeneralNote = general_note.to_string();
        self.set_has_general_note();
    }

    pub(crate) fn has_general_note<'a>(&self) -> &'a str {
        if self.HasGeneralNote {
            "Oui"
        } else {
            "Non"
        }
    }
    pub(crate) fn has_allergies<'a>(&self) -> &'a str {
        if self.HasAllergies {
            "Oui"
        } else {
            "Non"
        }
    }

    pub(crate) fn encrypt(&self) -> Self {
        let mut bene = self.clone();
        bene.Phone = encrypt(bene.Phone.as_bytes());
        bene.Email = encrypt(&bene.Email.as_bytes());
        bene.Address = encrypt(&bene.Address.as_bytes());
        bene.PostalCode = encrypt(&bene.PostalCode.as_bytes());
        bene.TsNote = encrypt(&bene.TsNote.as_bytes());
        bene.AdminNote = encrypt(&bene.AdminNote.as_bytes());
        bene.GeneralNote = encrypt(&bene.GeneralNote.as_bytes());
        bene.Situation = encrypt(&bene.Situation.as_bytes());
        bene
    }

    pub(crate) fn decrypt(&mut self) {
        self.Phone = String::from_utf8_lossy(&self.Phone.as_bytes()).to_string();
        self.Email = String::from_utf8_lossy(&self.Email.as_bytes()).to_string();
        self.Address = String::from_utf8_lossy(&self.Address.as_bytes()).to_string();
        self.PostalCode = String::from_utf8_lossy(&self.PostalCode.as_bytes()).to_string();
        self.TsNote = String::from_utf8_lossy(&self.TsNote.as_bytes()).to_string();
        self.AdminNote = String::from_utf8_lossy(&self.AdminNote.as_bytes()).to_string();
        self.GeneralNote = String::from_utf8_lossy(&self.GeneralNote.as_bytes()).to_string();
        self.Situation = String::from_utf8_lossy(&self.Situation.as_bytes()).to_string();
    }
}
