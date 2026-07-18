#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui::{self, Button, Color32, RichText};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 250.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Tempus",
        options,
        Box::new(|_cc| Ok(Box::<Tempus>::default())),
    )
}

enum SessionType {
    Focus,
    Break,
}

struct Tempus {
    tab: Tab,
    time_left: u32,
    is_running: bool,
    session_type: SessionType,
    session_count: u8,
    timer_settings: TimerSettings,
}

impl Default for Tempus {
    fn default() -> Self {
        let focus_duration = 25 * 60;

        Self {
            tab: Tab::Timer,
            time_left: focus_duration,
            is_running: false,
            session_type: SessionType::Focus,
            session_count: 0,
            timer_settings: TimerSettings {
                focus_duration,
                short_break_duration: 5 * 60,
                long_break_duration: 15 * 60,
                rounds: 4,
            },
        }
    }
}

struct TimerSettings {
    focus_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
    rounds: u8,
}

#[derive(PartialEq)]
enum Tab {
    Timer,
    Settings,
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
        ui.label(
            RichText::new(format!(
                "00:04:12 {}",
                match self.session_type {
                    SessionType::Focus => "Focus",
                    SessionType::Break => "Break",
                }
            ))
            .color(match self.session_type {
                SessionType::Focus => Color32::from_rgb(250, 82, 82),
                SessionType::Break => Color32::from_rgb(64, 192, 87),
            })
            .size(20.0),
        );

        ui.label(format!(
            "Number of completed sessions: {}",
            self.session_count
        ));

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                todo!();
            }

            if ui
                .add_enabled(!self.is_running, Button::new("Start"))
                .clicked()
            {
                todo!();
            }

            if ui
                .add_enabled(self.is_running, Button::new("Stop"))
                .clicked()
            {
                todo!();
            }

            if ui.button("Skip").clicked() {
                todo!();
            }
        });
    }

    fn settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.label("settings");
    }
}
