use crate::model::beneficiaries::beneficiary::{Beneficiaries, Beneficiary};
use crate::model::stats::statistiques::age::Ages;
use crate::model::stats::statistiques::city::Cities;
use crate::model::stats::statistiques::employment::Employments;
use crate::model::stats::statistiques::family_situation::FamilySituations;
use crate::model::stats::statistiques::income::Incomes;
use crate::model::stats::statistiques::kid::Kids;
use crate::model::stats::statistiques::language::Languages;
use crate::model::stats::statistiques::origin::Origins;
use crate::model::stats::statistiques::presence::Presences;
use crate::model::stats::statistiques::sexe::Sexes;
use crate::model::stats::statistiques::statistiques::Statistiques;
use crate::model::stats::statistiques::study::Studies;
use crate::model::stats::stats::{DateData, Stats};
use crate::model::stats::traits::Filterable;
use dioxus_hooks::UseSharedState;
use rust_xlsxwriter::{ColNum, Format, IntoExcelData, RowNum, Workbook, Worksheet, XlsxError};

struct ExcelHeader {
    headers: Vec<String>,
}
impl IntoExcelData for ExcelHeader {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        for (i, d) in self.headers.into_iter().enumerate() {
            worksheet.write(row, col + i as u16, d)?;
        }
        Ok(worksheet)
    }
    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        for (i, d) in self.headers.into_iter().enumerate() {
            worksheet.write_with_format(row, col + i as u16, d, format)?;
        }
        Ok(worksheet)
    }
}

impl ExcelHeader {
    fn new(header: &str) -> Self {
        let values = match header {
            "Beneficiary" => vec![
                "Prénom".to_string(),
                "Nom".to_string(),
                "Date de naissance".to_string(),
                "Ville".to_string(),
                "Téléphone".to_string(),
                "Email".to_string(),
                "Dernière présence".to_string(),
                "Adulte".to_string(),
                "Enfant".to_string(),
                "Actif".to_string(),
            ],
            "Statistiques" => vec![
                "Total".to_string(),
                "Actif".to_string(),
                "Visites".to_string(),
                "Age".to_string(),
                "Sexe".to_string(),
                "Langue".to_string(),
                "Origine".to_string(),
                "Enfant".to_string(),
                "Situation familiale".to_string(),
                "Ville".to_string(),
                "Etude".to_string(),
                "Emploi".to_string(),
                "Revenu".to_string(),
            ],
            "Présences" => vec![
                "Date".to_string(),
                "Total".to_string(),
                "Actif".to_string(),
                "Visites".to_string(),
            ],
            "Ages" => vec![
                "Date".to_string(),
                "0-19 ans".to_string(),
                "20-29 ans".to_string(),
                "30-39 ans".to_string(),
                "40-49 ans".to_string(),
                "50-59 ans".to_string(),
                "60-69 ans".to_string(),
                "70+ ans".to_string(),
            ],
            "Villes" => vec![
                "Date".to_string(),
                "Carignan".to_string(),
                "Chambly".to_string(),
                "Marieville".to_string(),
                "Richelieu".to_string(),
                "St-Mathias".to_string(),
                "Autre".to_string(),
            ],
            "Emplois" => vec![
                "Date".to_string(),
                "Aucun emploi".to_string(),
                "Employé".to_string(),
            ],
            "Situation familiale" => vec![
                "Date".to_string(),
                "Célibataire".to_string(),
                "Couple".to_string(),
                "Couple avec enfants".to_string(),
                "Recomposée".to_string(),
                "Monoparental".to_string(),
                "Autre".to_string(),
            ],
            "Revenus" => vec![
                "Date".to_string(),
                "Pas de revenus".to_string(),
                "1$ à 14,999$".to_string(),
                "15,000$ à 29,999$".to_string(),
                "30,000$ et plus".to_string(),
            ],
            "Enfants" => vec![
                "Date".to_string(),
                "0 enfants".to_string(),
                "1 enfants".to_string(),
                "2 enfants".to_string(),
                "3-4 enfants".to_string(),
                "5+ enfants".to_string(),
            ],
            "Langues" => vec![
                "Date".to_string(),
                "Français".to_string(),
                "Anglais".to_string(),
                "Espagnol".to_string(),
                "Arabe".to_string(),
                "Mandarin".to_string(),
                "Autre".to_string(),
            ],
            "Origines" => vec![
                "Date".to_string(),
                "Amérique du Nord".to_string(),
                "Amérique du Sud".to_string(),
                "Amérique central".to_string(),
                "Asie".to_string(),
                "Afrique".to_string(),
                "Europe".to_string(),
                "Autre".to_string(),
            ],
            "Sexes" => vec![
                "Date".to_string(),
                "Homme".to_string(),
                "Femme".to_string(),
                "Autre".to_string(),
            ],
            "Etudes" => vec![
                "Date".to_string(),
                "Aucune scolarité".to_string(),
                "Primaire".to_string(),
                "Secondaire".to_string(),
                "Collégial".to_string(),
                "Universitaire".to_string(),
                "Autre".to_string(),
            ],
            _ => vec![],
        };
        Self { headers: values }
    }
}

fn write_vec(
    data: Vec<DateData>,
    worksheet: &mut Worksheet,
    row: RowNum,
    col: ColNum,
) -> Result<&mut Worksheet, XlsxError> {
    for (i, d) in data.into_iter().enumerate() {
        worksheet.write(row, col + i as u16, d)?;
    }
    Ok(worksheet)
}

fn write_vec_with_format<'a>(
    data: Vec<DateData>,
    worksheet: &'a mut Worksheet,
    row: RowNum,
    col: ColNum,
    format: &Format,
) -> Result<&'a mut Worksheet, XlsxError> {
    for (i, d) in data.into_iter().enumerate() {
        worksheet.write_with_format(row, col + i as u16, d, format)?;
    }
    Ok(worksheet)
}

impl Stats {
    pub(crate) fn to_worksheets(
        &self,
        workbook: &mut Workbook,
        beneficiaries: UseSharedState<Beneficiaries>,
    ) -> Result<(), XlsxError> {
        Self::create_worksheets(workbook, self, beneficiaries)?;
        Ok(())
    }

    fn create_worksheets(
        workbook: &mut Workbook,
        stats: &Stats,
        beneficiaries: UseSharedState<Beneficiaries>,
    ) -> Result<(), XlsxError> {
        let statistics = Statistiques::from(stats);
        Self::create_worksheet(
            workbook,
            "Bénéficiaires",
            beneficiaries.read().clone(),
            ExcelHeader::new("Beneficiary"),
        )?;
        Self::create_worksheet(
            workbook,
            "Statistiques",
            statistics,
            ExcelHeader::new("Statistiques"),
        )?;
        Self::create_worksheet(
            workbook,
            "Présences",
            stats.presences.clone(),
            ExcelHeader::new("Présences"),
        )?;
        Self::create_worksheet(
            workbook,
            "Ages",
            stats.ages.clone(),
            ExcelHeader::new("Ages"),
        )?;
        Self::create_worksheet(
            workbook,
            "Villes",
            stats.cities.clone(),
            ExcelHeader::new("Villes"),
        )?;
        Self::create_worksheet(
            workbook,
            "Emplois",
            stats.employments.clone(),
            ExcelHeader::new("Emplois"),
        )?;
        Self::create_worksheet(
            workbook,
            "Situation familiale",
            stats.familySituations.clone(),
            ExcelHeader::new("Situation familiale"),
        )?;
        Self::create_worksheet(
            workbook,
            "Revenus",
            stats.incomes.clone(),
            ExcelHeader::new("Revenus"),
        )?;
        Self::create_worksheet(
            workbook,
            "Enfants",
            stats.kids.clone(),
            ExcelHeader::new("Enfants"),
        )?;
        Self::create_worksheet(
            workbook,
            "Langues",
            stats.languages.clone(),
            ExcelHeader::new("Langues"),
        )?;
        Self::create_worksheet(
            workbook,
            "Origines",
            stats.origins.clone(),
            ExcelHeader::new("Origines"),
        )?;
        Self::create_worksheet(
            workbook,
            "Sexes",
            stats.sexes.clone(),
            ExcelHeader::new("Sexes"),
        )?;
        Self::create_worksheet(
            workbook,
            "Etudes",
            stats.studies.clone(),
            ExcelHeader::new("Etudes"),
        )?;

        Ok(())
    }
    fn create_worksheet(
        workbook: &mut Workbook,
        name: &str,
        data: impl IntoExcelData,
        header: ExcelHeader,
    ) -> Result<(), XlsxError> {
        let worksheet = workbook.add_worksheet();
        worksheet.set_name(name)?;
        worksheet.write(0, 0, header)?;
        data.write(worksheet, 1, 0)?;

        Ok(())
    }
}

impl IntoExcelData for DateData {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        worksheet.write(row, 0, self.date.to_string())?;
        for (i, d) in self.values.into_iter().enumerate() {
            worksheet.write(row, col + 1 + i as u16, d)?;
        }
        Ok(worksheet)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        worksheet.write_datetime_with_format(row, 0, self.date, format)?;
        for (i, d) in self.values.into_iter().enumerate() {
            worksheet.write_with_format(row, col + 1 + i as u16, d, format)?;
        }
        Ok(worksheet)
    }
}

impl IntoExcelData for Presences {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Ages {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Cities {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Employments {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for FamilySituations {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Incomes {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Kids {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Languages {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Origins {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Sexes {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Studies {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        write_vec(self.get_data(), worksheet, row, col)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        write_vec_with_format(self.get_data(), worksheet, row, col, format)
    }
}

impl IntoExcelData for Beneficiaries {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        for (i, beneficiary) in self.beneficiaries.into_iter().enumerate() {
            worksheet.write(row + i as u32, col, beneficiary)?;
        }
        Ok(worksheet)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        for (i, beneficiary) in self.beneficiaries.into_iter().enumerate() {
            worksheet.write_with_format(row + i as u32, col, beneficiary, format)?;
        }
        Ok(worksheet)
    }
}

impl IntoExcelData for Beneficiary {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        worksheet.write(row, col, self.FirstName)?;
        worksheet.write(row, col + 1, self.LastName)?;
        worksheet.write(row, col + 2, self.Birth)?;
        worksheet.write(row, col + 3, self.City)?;
        worksheet.write(row, col + 4, self.Phone)?;
        worksheet.write(row, col + 5, self.Email)?;
        worksheet.write(row, col + 6, self.LastPresence)?;
        worksheet.write(row, col + 7, self.Adult)?;
        worksheet.write(row, col + 8, self.Kid)?;
        worksheet.write(row, col + 9, self.IsActive)?;
        Ok(worksheet)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        worksheet.write_with_format(row, col, self.FirstName, format)?;
        worksheet.write_with_format(row, col + 1, self.LastName, format)?;
        worksheet.write_with_format(row, col + 2, self.Birth, format)?;
        worksheet.write_with_format(row, col + 3, self.City, format)?;
        worksheet.write_with_format(row, col + 4, self.Phone, format)?;
        worksheet.write_with_format(row, col + 5, self.Email, format)?;
        worksheet.write_with_format(row, col + 6, self.LastPresence, format)?;
        worksheet.write_with_format(row, col + 7, self.Adult, format)?;
        worksheet.write_with_format(row, col + 8, self.Kid, format)?;
        worksheet.write_with_format(row, col + 9, self.IsActive, format)?;

        Ok(worksheet)
    }
}

impl IntoExcelData for Statistiques {
    fn write(
        self,
        worksheet: &mut Worksheet,
        row: RowNum,
        col: ColNum,
    ) -> Result<&mut Worksheet, XlsxError> {
        worksheet.write(row, col, self.total)?;
        worksheet.write(row, col + 1, self.active)?;
        worksheet.write(row, col + 2, self.visits)?;
        worksheet.write(row, col + 3, self.age)?;
        worksheet.write(row, col + 4, self.sexe)?;
        worksheet.write(row, col + 5, self.language)?;
        worksheet.write(row, col + 6, self.origin)?;
        worksheet.write(row, col + 7, self.kid)?;
        worksheet.write(row, col + 8, self.family_situation)?;
        worksheet.write(row, col + 9, self.city)?;
        worksheet.write(row, col + 10, self.study)?;
        worksheet.write(row, col + 11, self.employment)?;
        worksheet.write(row, col + 12, self.income)?;

        Ok(worksheet)
    }

    fn write_with_format<'a>(
        self,
        worksheet: &'a mut Worksheet,
        row: RowNum,
        col: ColNum,
        format: &'a Format,
    ) -> Result<&'a mut Worksheet, XlsxError> {
        worksheet.write_with_format(row, col, self.total, format)?;
        worksheet.write_with_format(row, col + 1, self.active, format)?;
        worksheet.write_with_format(row, col + 2, self.visits, format)?;
        worksheet.write_with_format(row, col + 3, self.age, format)?;
        worksheet.write_with_format(row, col + 4, self.sexe, format)?;
        worksheet.write_with_format(row, col + 5, self.language, format)?;
        worksheet.write_with_format(row, col + 6, self.origin, format)?;
        worksheet.write_with_format(row, col + 7, self.kid, format)?;
        worksheet.write_with_format(row, col + 8, self.family_situation, format)?;
        worksheet.write_with_format(row, col + 9, self.city, format)?;
        worksheet.write_with_format(row, col + 10, self.study, format)?;
        worksheet.write_with_format(row, col + 11, self.employment, format)?;
        worksheet.write_with_format(row, col + 12, self.income, format)?;

        Ok(worksheet)
    }
}
