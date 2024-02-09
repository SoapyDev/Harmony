#![windows_subsystem = "windows"]

use dioxus_desktop::WindowCloseBehaviour::LastWindowExitsApp;
use dioxus_desktop::{Config, LogicalSize, WindowBuilder, WindowCloseBehaviour};
use std::fs;
use std::path::Path;
use dioxus_desktop::tao::platform::windows::{IconExtWindows, WindowBuilderExtWindows};
use dioxus_desktop::tao::window::Icon;

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
                    .with_title("Harmonie")
                    .with_maximized(true)
                    .with_focused(true)
                    .with_content_protection(true)
                    .with_min_inner_size(LogicalSize::new(700, 800))
                    .with_taskbar_icon(Icon::from_path("./src/assets/exportable_icons/icon.png", None).ok())
                    .with_window_icon(Icon::from_path("./src/assets/exportable_icons/icon.png", None).ok())
                    .with_theme(Some(dioxus_desktop::tao::window::Theme::Dark)),
            )
            .with_background_color((20, 34, 64, 200))
            .with_prerendered(String::from("<h1>Initialisation</h1>"))
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
