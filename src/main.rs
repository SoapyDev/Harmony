#![allow(non_snake_case)]

use dioxus_desktop::WindowCloseBehaviour::LastWindowExitsApp;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
use std::fs;
use std::path::Path;

mod app;
mod controler;
pub(crate) mod model;
mod view;

fn main() {
    clear_graphs();
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


fn clear_graphs() {
    let path = "./output/graph/";
    if Path::new(path).exists() {
        fs::read_dir(path).unwrap().for_each(|x| {
            let _ = fs::remove_file(x.unwrap().path());
        });
    }
}
fn close_window() -> WindowCloseBehaviour {
    clear_graphs();
    LastWindowExitsApp
}
