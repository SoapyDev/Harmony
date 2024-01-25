#![allow(non_snake_case)]

use chrono::Local;
use dioxus_desktop::WindowCloseBehaviour::LastWindowExitsApp;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
use log::{info, LevelFilter};
use simple_logging::log_to_file;
use std::fs;
use std::fs::File;
use std::path::Path;

mod app;
mod controler;
pub(crate) mod model;
mod view;

fn main() {
    clear_graphs();
    if let Err(e) = log_to_file(create_log_file(), LevelFilter::Info) {
        eprintln!("Could not initiate logging due to error : {:?}", e);
        log_to_file("public/logs/log.log", LevelFilter::Info).expect("Could not initiate logging");
    };
    info!("Initiating application");
    dioxus_desktop::launch_cfg(
        app::app,
        Config::default()
            .with_window(
                WindowBuilder::new()
                    .with_title("Harmony")
                    .with_maximized(true)
                    .with_min_inner_size(LogicalSize::new(650, 750)),
            )
            .with_background_color((20, 34, 64, 200))
            .with_data_directory("./data")
            .with_resource_directory("./output")
            .with_disable_context_menu(true)
            .with_close_behaviour(close_window()),
    );
}

fn create_log_file() -> String {
    let now = Local::now();
    let date = now.format("%Y-%m-%d");
    let path = format!("./output/log/{}.log", date);
    if !Path::new(&path).exists() {
        let _ = File::create(&path);
    }
    path
}

fn clear_graphs() {
    let path = "./output/graph/";
    if Path::new(path).exists() {
        fs::read_dir(path).unwrap().for_each(|x| {
            let _ = fs::remove_file(x.unwrap().path());
        });
    }
}
fn close_window() -> WindowCloseBehaviour {
    info!("Closing application");
    clear_graphs();
    LastWindowExitsApp
}
