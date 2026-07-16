#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default(); 
    eframe::run_native("Tempus", options, Box::new(|_cc| {
        Ok(Box::<Tempus>::default())
    }))
}

struct Tempus {
    test: String
}

impl Default for Tempus {
    fn default() -> Self {
       Self {
        test: String::new()
       } 
    }
}

impl eframe::App for Tempus {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Test App");
        });
    }
}