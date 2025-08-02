mod audio;

use crate::audio::audio;
use eframe::egui;
use egui::{Color32, ProgressBar};
use std::time::{Duration, Instant};

enum State {
    MainState,
    SettingsState,
    AudioState,
}

struct Settings {
    time: u32,
}

struct Progress {
    progress: f32,
    last_ticker_update: Instant,
}

struct RustickApp {
    state: State,
    settings: Settings,
    progress: Progress,
}

impl Default for RustickApp {
    fn default() -> Self {
        Self {
            state: State::MainState,
            settings: Settings { time: 10 },
            progress: Progress {
                progress: 0.0,
                last_ticker_update: Instant::now(),
            },
        }
    }
}

impl RustickApp {
    fn update_progress(&mut self) {
        self.progress.progress += 1.0
    }

    fn calculate_current_progress(&mut self) -> f32 {
        (self.progress.progress / self.settings.time as f32 * 100.0).floor() / 100.0
    }
}

impl eframe::App for RustickApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            let formatted_label = format!("{}/{}", self.progress.progress, self.settings.time);
            ui.label(formatted_label);

            let progress_bar = ProgressBar::new(self.calculate_current_progress())
                .fill(Color32::BLUE)
                .show_percentage();

            ui.add(progress_bar);

            if ui.button("Go to Settings").clicked() {
                self.state = State::SettingsState;
            }

            if let State::SettingsState = self.state {
                ui.label("Settings");
                ui.add(egui::Slider::new(&mut self.settings.time, 0..=180).text("Duration"));
                if ui.button("Back to Main Screen").clicked() {
                    self.state = State::MainState;
                }
            }
        });

        if self.progress.last_ticker_update.elapsed() > Duration::from_millis(950) {
            self.update_progress();
            self.progress.last_ticker_update = Instant::now();
        };

        match self.state {
            State::MainState => {
                if self.progress.progress > self.settings.time as f32 {
                    self.state = State::AudioState;
                    self.progress.progress = 0.0;
                }
            }
            State::SettingsState => {}
            State::AudioState => {
                std::thread::spawn(audio);
                self.state = State::MainState;
            }
        }

        ctx.request_repaint_after_secs(1.0);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((200.0, 100.0)),
        ..eframe::NativeOptions::default()
    };

    let app = RustickApp::default();
    eframe::run_native("rustick", native_options, Box::new(|_cc| Ok(Box::new(app))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_calculation_half() {
        let mut app = RustickApp::default();
        app.progress.progress = 30.0;
        app.settings.time = 60;
        let progress = app.calculate_current_progress();
        assert_eq!(progress, 0.50);
    }
    #[test]
    fn test_progress_calculation_one_third() {
        let mut app = RustickApp::default();
        app.progress.progress = 20.0;
        app.settings.time = 60;
        let progress = app.calculate_current_progress();
        assert_eq!(progress, 0.33);
    }
}
