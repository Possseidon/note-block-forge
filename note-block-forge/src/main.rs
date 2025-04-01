// #![allow(dead_code)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod about;
mod action;
mod app;
mod menu;
mod midi_devices;
mod widgets;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        app::App::NAME,
        Default::default(),
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
