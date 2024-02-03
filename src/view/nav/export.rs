use crate::model::beneficiaries::beneficiary::Beneficiaries;
use crate::model::stats::stats::Stats;
use dioxus::prelude::*;
use dioxus_hooks::use_shared_state;
use rust_xlsxwriter::Workbook;
use std::fs;
use std::path::PathBuf;

pub(crate) fn export(cx: Scope) -> Result<(), anyhow::Error> {
    let beneficiaries = use_shared_state::<Beneficiaries>(cx).unwrap();
    let path = make_folder()?;
    copy_logos(path.clone())?;
    copy_graphs(path.clone())?;

    let file_path = make_file_path(path);
    let mut workbook = Workbook::new();
    let stats = use_shared_state::<Stats>(cx).unwrap();
    stats
        .read()
        .to_worksheets(&mut workbook, beneficiaries.clone())?;
    workbook.save(&file_path)?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn make_folder() -> Result<PathBuf, anyhow::Error> {
    let current_user = std::env::var("USER")?;
    let current_date = chrono::Local::now().format("%d-%m-%Y_%H-%M-%S");
    let path = format!("/home/{}/Downloads/Harmony/{}", current_user, current_date);
    let path = PathBuf::from(path);
    if !path.exists() {
        fs::create_dir_all(path.clone())?;
    }
    Ok(path)
}

#[cfg(target_os = "windows")]
fn make_folder() -> Result<PathBuf, anyhow::Error> {
    let current_user = std::env::var("USER")?;
    let current_date = chrono::Local::now().format("%d-%m-%Y_%H-%M-%S");
    let path = format!(
        "C:\\Users\\{}\\Downloads\\Harmony\\{}",
        current_user, current_date
    );
    let path = PathBuf::from(path);
    if !path.exists() {
        fs::create_dir_all(path.clone())?;
    }
    Ok(path)
}

fn make_file_path(mut path: PathBuf) -> PathBuf {
    path.push("data.xlsx");
    path
}

fn copy_graphs(path: PathBuf) -> Result<(), anyhow::Error> {
    let path = path.join("graphs");
    copy_files(PathBuf::from("./output/graph"), path)
}

fn copy_logos(path: PathBuf) -> Result<(), anyhow::Error> {
    let path = path.join("logos");
    copy_files(PathBuf::from("./output/icons"), path)
}

fn copy_files(origin: PathBuf, destination: PathBuf) -> Result<(), anyhow::Error> {
    if !destination.exists() {
        fs::create_dir_all(destination.clone())?;
    }
    for file in fs::read_dir(origin)? {
        let file = file?;
        fs::copy(file.path(), destination.join(file.file_name()))?;
    }
    Ok(())
}
