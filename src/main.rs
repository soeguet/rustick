mod audio;

use crate::audio::AudioMaster;
use eframe::egui;
use egui::{Color32, ProgressBar, Ui, Vec2, ViewportCommand};
use std::time::{Duration, Instant};

const TICKER_SCREEN_WIDTH: f32 = 74.0;
const TICKER_SCREEN_HEIGHT: f32 = 200.0;
const SETTINGS_SCREEN_WIDTH: f32 = 500.0;
const SETTINGS_SCREEN_HEIGHT: f32 = 500.0;
const TICKER_START_COUNT: f32 = 0.0;
const TICKER_INTERVAL: f32 = 1.0;
const SPACING: f32 = 5.0;

enum State {
    MainState,
    SettingsState,
    BreakState,
    AudioState,
}

struct Settings {
    time: u32,
    break_time: u32,
    prev_state: State,
}

struct Progress {
    progress: f32,
    last_ticker_update: Instant,
}

struct RustickApp {
    message: Option<(String, Instant)>,
    state: State,
    next_state: State,
    settings: Settings,
    progress: Progress,
    audio_master: AudioMaster,
}

impl Default for RustickApp {
    fn default() -> Self {
        Self {
            message: None,
            state: State::MainState,
            next_state: State::BreakState,
            settings: Settings {
                time: 10,
                break_time: 5,
                prev_state: State::MainState,
            },
            progress: Progress {
                progress: TICKER_START_COUNT,
                last_ticker_update: Instant::now(),
            },
            audio_master: AudioMaster::default(),
        }
    }
}

impl RustickApp {
    fn update_progress(&mut self) {
        self.progress.progress += TICKER_INTERVAL;
    }

    fn render_go_to_settings_button(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        prev_state: State,
    ) {
        if ui.button("Go to Settings").clicked() {
            self.audio_master.pre_settings_phase();
            self.state = State::SettingsState;
            self.settings.prev_state = prev_state;

            let size: Vec2 = Vec2 {
                x: SETTINGS_SCREEN_WIDTH,
                y: SETTINGS_SCREEN_HEIGHT,
            };
            ctx.send_viewport_cmd(ViewportCommand::InnerSize(size));
        }
    }

    fn calculate_current_progress(&mut self, time: f32) -> f32 {
        (self.progress.progress / time * 100.0).floor() / 100.0
    }
    fn evaluate_progress(&mut self) {
        if self.progress.last_ticker_update.elapsed() > Duration::from_millis(950) {
            self.update_progress();
            self.progress.last_ticker_update = Instant::now();
        };
    }

    fn render_settings_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            // top settings label
            ui.label(
                egui::RichText::new("Settings")
                    .extra_letter_spacing(2.0)
                    .size(25.0)
                    .underline(),
            );

            // errors
            if let Some((msg, since)) = &self.message {
                if since.elapsed().as_secs_f32() < 3.0 {
                    ui.add_space(SPACING);
                    ui.separator();
                    ui.label(
                        egui::RichText::new(msg.clone())
                            .color(Color32::RED)
                            .underline(),
                    );
                    ui.separator();
                    ui.add_space(SPACING);
                } else {
                    self.message = None;
                }
            }

            // slide main
            ui.add_space(SPACING);
            ui.label("Change time - Main");
            ui.add(egui::Slider::new(&mut self.settings.time, 0..=600).text("Duration"));

            // main audio path
            ui.add_space(SPACING);
            ui.label("Change main audio path");
            ui.add(egui::TextEdit::singleline(
                &mut self.audio_master.main_audio.get_mut().audio_path,
            ));
            if ui.button("save new audio file").clicked() {
                let borrowed_main_audio = &self.audio_master.main_audio.get_mut();
                let equal = borrowed_main_audio.audio_path != borrowed_main_audio.audio_path_int;

                if equal {
                    let ref_mut = self.audio_master.main_audio.get_mut();
                    let result = self.audio_master.set_main_audio_path();

                    match result {
                        Ok(_) => {
                            self.message =
                                Some(("Changes were applied".to_string(), Instant::now()));
                        }
                        Err(err) => {
                            self.message = Some((format!("{}", err.message), Instant::now()));
                        }
                    }
                } else {
                    self.message = Some(("The path is the same...".to_string(), Instant::now()));
                }
            }
            ui.add_space(SPACING);
            ui.separator();

            // slider for break
            ui.add_space(SPACING);
            ui.label("Change time - Break");
            ui.add(
                egui::Slider::new(&mut self.settings.break_time, 0..=180).text("Break Duration"),
            );

            // break audio path
            ui.add_space(SPACING);
            ui.label("Change main audio path");
            ui.add(egui::TextEdit::singleline(
                &mut self.audio_master.break_audio.get_mut().audio_path,
            ));
            if ui.button("save new audio file (break)").clicked() {
                let equal = self.audio_master.break_audio.borrow().audio_path
                    != self.audio_master.break_audio.borrow().audio_path_int;

                if equal {
                    let result = self.audio_master.set_main_audio_path();

                    match result {
                        Ok(_) => {
                            self.message =
                                Some(("Changes were applied".to_string(), Instant::now()));
                        }
                        Err(err) => {
                            self.message = Some((format!("{}", err.message), Instant::now()));
                        }
                    }
                } else {
                    self.message = Some(("The path is the same...".to_string(), Instant::now()));
                }
            }
            ui.add_space(SPACING);
            ui.separator();

            ui.add_space(SPACING);
            ui.separator();
            // back button
            ui.add_space(3.0 * SPACING);
            if ui.button("Back to Main Screen").clicked() {
                match &self.settings.prev_state {
                    State::MainState => {
                        self.state = State::MainState;
                    }
                    State::BreakState => {
                        self.state = State::BreakState;

                        self.settings.prev_state = State::MainState;
                    }
                    _ => {}
                };

                let size: Vec2 = Vec2 {
                    x: TICKER_SCREEN_HEIGHT,
                    y: TICKER_SCREEN_WIDTH,
                };
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(size));
            }
        });
    }

    fn render_main_screen(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            let formatted_label = format!("{}/{}", self.progress.progress, self.settings.time);
            ui.label(formatted_label);

            let progress_bar =
                ProgressBar::new(self.calculate_current_progress(self.settings.time as f32))
                    .fill(Color32::BLUE)
                    .show_percentage();

            ui.add(progress_bar);

            self.render_go_to_settings_button(ui, ctx, State::MainState);
        });

        self.evaluate_progress();
    }

    fn render_break_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            let formatted_label =
                format!("{}/{}", self.progress.progress, self.settings.break_time);
            ui.label(formatted_label);

            let progress_bar =
                ProgressBar::new(self.calculate_current_progress(self.settings.break_time as f32))
                    .fill(Color32::DARK_GREEN)
                    .show_percentage();

            ui.add(progress_bar);

            self.render_go_to_settings_button(ui, ctx, State::BreakState);
        });
        self.evaluate_progress();
    }
}

impl eframe::App for RustickApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        match self.state {
            State::MainState => {
                self.render_main_screen(ctx, frame);

                if self.progress.progress > self.settings.time as f32 {
                    self.state = State::AudioState;
                    self.progress.progress = TICKER_START_COUNT;
                }
            }
            State::SettingsState => {
                self.render_settings_screen(ctx);
            }
            State::AudioState => {
                match self.next_state {
                    State::MainState => {
                        self.audio_master.run_main_audio();
                        self.state = State::MainState;
                        self.next_state = State::BreakState;
                    }
                    State::BreakState => {
                        self.audio_master.run_break_audio();
                        self.state = State::BreakState;
                        self.next_state = State::MainState;
                    }
                    _ => {}
                };
            }
            State::BreakState => {
                self.render_break_screen(ctx);
                if self.progress.progress > self.settings.break_time as f32 {
                    self.state = State::AudioState;
                    self.progress.progress = TICKER_START_COUNT;
                }
            }
        }

        ctx.request_repaint_after_secs(TICKER_INTERVAL);
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size((TICKER_SCREEN_HEIGHT, TICKER_SCREEN_WIDTH)),
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
        let progress = app.calculate_current_progress(app.settings.time as f32);
        assert_eq!(progress, 0.50);
    }
    #[test]
    fn test_progress_calculation_one_third() {
        let mut app = RustickApp::default();
        app.progress.progress = 20.0;
        app.settings.time = 60;
        let progress = app.calculate_current_progress(app.settings.time as f32);
        assert_eq!(progress, 0.33);
    }
}
