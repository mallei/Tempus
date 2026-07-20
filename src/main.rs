#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use actually_beep::beep_with_hz_and_millis;
use eframe::egui::{self, Button, Color32, RichText};
use notify_rust::Notification;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("tempus")
            .with_inner_size([300.0, 250.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Tempus",
        options,
        Box::new(|cc| Ok(Box::new(Tempus::new(cc)))),
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
    last_update: Instant,
}

impl Tempus {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let timer_settings = cc
            .storage
            .and_then(|s| eframe::get_value::<TimerSettings>(s, "settings"))
            .unwrap_or_else(|| TimerSettings {
                focus_duration: 25 * 60,
                short_break_duration: 5 * 60,
                long_break_duration: 15 * 60,
                dark_mode: true,
                notifications: true,
                rounds: 4,
            });

        Self {
            tab: Tab::Timer,
            time_left: timer_settings.focus_duration,
            is_running: false,
            session_type: SessionType::Focus,
            session_count: 0,
            timer_settings,
            last_update: Instant::now(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TimerSettings {
    focus_duration: u32,
    short_break_duration: u32,
    long_break_duration: u32,
    rounds: u8,
    dark_mode: bool,
    notifications: bool,
}

#[derive(PartialEq)]
enum Tab {
    Timer,
    Settings,
}

impl Tempus {
    fn is_long_break(&self) -> bool {
        (self.session_count + 1) % self.timer_settings.rounds == 0
    }

    fn switch_session(&mut self) {
        match self.session_type {
            SessionType::Focus => {
                self.time_left = if (self.session_count + 1) % self.timer_settings.rounds == 0 {
                    if self.timer_settings.notifications {
                        Notification::new()
                            .body("Time to take a long break!")
                            .show()
                            .unwrap();
                    }
                    self.timer_settings.long_break_duration
                } else {
                    if self.timer_settings.notifications {
                        Notification::new()
                            .body("Time to take a short break!")
                            .show()
                            .unwrap();
                    }
                    self.timer_settings.short_break_duration
                };
                self.session_type = SessionType::Break;
            }
            SessionType::Break => {
                self.session_count += 1;
                if self.timer_settings.notifications {
                    Notification::new().body("Time to focus!").show().unwrap();
                }
                self.time_left = self.timer_settings.focus_duration;
                self.session_type = SessionType::Focus;
            }
        }
    }
}

impl eframe::App for Tempus {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "settings", &self.timer_settings);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_running {
            let elapsed = self.last_update.elapsed();
            let secs = elapsed.as_secs() as u32;

            if secs > 0 {
                self.time_left = self.time_left.saturating_sub(secs);
                self.last_update += Duration::from_secs(secs as u64);

                if self.time_left == 0 {
                    self.is_running = false;
                    beep_with_hz_and_millis(600, 450).unwrap();
                    self.switch_session();
                }
            }

            ui.request_repaint_after(Duration::from_millis(100));
        }

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
                "{} {}",
                format!(
                    "{:02}:{:02}:{:02}",
                    self.time_left / 3600,
                    (self.time_left % 3600) / 60,
                    self.time_left % 60
                ),
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
                self.is_running = false;
                self.time_left = match self.session_type {
                    SessionType::Focus => self.timer_settings.focus_duration,
                    SessionType::Break if self.is_long_break() => {
                        self.timer_settings.long_break_duration
                    }
                    SessionType::Break => self.timer_settings.short_break_duration,
                };
            }

            if ui
                .add_enabled(self.time_left > 0 && !self.is_running, Button::new("Start"))
                .clicked()
            {
                self.is_running = true;
                self.last_update = Instant::now();
            }

            if ui
                .add_enabled(self.is_running, Button::new("Stop"))
                .clicked()
            {
                self.is_running = false;
            }

            if ui.button("Skip").clicked() {
                self.is_running = false;
                self.switch_session();
            }
        });
    }

    fn settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.columns(2, |cols| {
            cols[0].vertical(|ui| {
                ui.label("Focus Duration (min)");
                {
                    let mut val = self.timer_settings.focus_duration / 60;
                    if ui.add(egui::Slider::new(&mut val, 1..=120)).changed() {
                        self.timer_settings.focus_duration = val * 60;
                        if !self.is_running && matches!(self.session_type, SessionType::Focus) {
                            self.time_left = self.timer_settings.focus_duration;
                        }
                    }
                }

                ui.add_space(8.0);

                ui.label("Short Break Duration (min)");
                {
                    let mut val = self.timer_settings.short_break_duration / 60;
                    if ui.add(egui::Slider::new(&mut val, 1..=60)).changed() {
                        self.timer_settings.short_break_duration = val * 60;
                        if !self.is_running
                            && !self.is_long_break()
                            && matches!(self.session_type, SessionType::Break)
                        {
                            self.time_left = self.timer_settings.short_break_duration;
                        }
                    }
                }

                ui.add_space(8.0);

                ui.label("Long Break Duration (min)");
                {
                    let mut val = self.timer_settings.long_break_duration / 60;
                    if ui.add(egui::Slider::new(&mut val, 1..=60)).changed() {
                        self.timer_settings.long_break_duration = val * 60;
                        if !self.is_running
                            && self.is_long_break()
                            && matches!(self.session_type, SessionType::Break)
                        {
                            self.time_left = self.timer_settings.long_break_duration;
                        }
                    }
                }

                ui.add_space(8.0);

                ui.label("Rounds");
                {
                    let mut val = self.timer_settings.rounds;
                    if ui.add(egui::Slider::new(&mut val, 1..=20)).changed() {
                        self.timer_settings.rounds = val;
                    }
                }
            });
            cols[1].vertical(|ui| {
                ui.checkbox(&mut self.timer_settings.notifications, "Notifications");

                ui.add_space(8.0);

                if ui
                    .checkbox(&mut self.timer_settings.dark_mode, "Dark Mode")
                    .changed()
                {
                    if self.timer_settings.dark_mode {
                        ui.set_theme(egui::Theme::Dark);
                    } else {
                        ui.set_theme(egui::Theme::Light);
                    }
                }
            });
        });
    }
}
