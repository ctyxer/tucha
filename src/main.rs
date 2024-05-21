#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

mod enums;
mod types;
mod ui;
mod utils;

use std::{env::set_current_dir, fs};

use dirs::data_local_dir;
use ui::window::Window;

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    if let Some(mut tucha_location) = data_local_dir() {
        tucha_location.push("tucha/sessions");

        if fs::create_dir_all(&tucha_location).is_err() {
            panic!("Failed to create a local folder on the path: {}", &tucha_location.display().to_string());
        }
        tucha_location.pop();
        if set_current_dir(&tucha_location).is_err() {
            panic!("Failed to set current directory in path: {}", &tucha_location.display().to_string());
        }
        

        dotenv::dotenv().ok();

        let options = eframe::NativeOptions {
            ..Default::default()
        };

        eframe::run_native("tucha", options, Box::new(|_cc| Box::<Window>::default()))
    } else {
        panic!("Failed to get local data directory. Exiting");
    }
}
