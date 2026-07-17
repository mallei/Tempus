#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default(); 
    eframe::run_native("Tempus", options, Box::new(|_cc| {
        Ok(Box::<Tempus>::default())
    }))
}

struct Tempus {
    tab: Tab
}

impl Default for Tempus {
    fn default() -> Self {
       Self {
        tab: Tab::Timer
       } 
    }
}

#[derive(PartialEq)]
enum Tab {
    Timer,
    Settings
}

impl eframe::App for Tempus {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.tab, Tab::Timer, "⏰ Timer");
                ui.selectable_value(&mut self.tab, Tab::Settings, "⛭ Settings");
            });

            ui.separator();

            match self.tab {
                Tab::Timer => self.timer_tab(ui),
                Tab::Settings => self.settings_tab(ui),
            }
        });
    }
}

impl Tempus {
    fn timer_tab(&mut self, ui: &mut egui::Ui) {
        ui.label("timer");
    }

    fn settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.label("settings");
    }
}