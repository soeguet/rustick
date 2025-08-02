use eframe::egui;
use egui::{Color32, ProgressBar};
use std::time::{Duration, Instant};

enum Screen {
    MainScreen,
    SettingsScreen,
}

struct Settings {
    time: u32,
}

struct Progress {
    progress: f32,
    last_ticker_update: Instant,
}

struct TickaApp {
    screen: Screen,
    settings: Settings,
    progress: Progress,
}

impl Default for TickaApp {
    fn default() -> Self {
        Self {
            screen: Screen::MainScreen,
            settings: Settings { time: 60 },
            progress: Progress {
                progress: 0.0,
                last_ticker_update: Instant::now(),
            },
        }
    }
}

impl TickaApp {
    fn update_progress(&mut self) {
        self.progress.progress += 1.0
    }

    fn calculate_current_progress(&mut self) -> f32 {
        if self.progress.progress == 0.0 {
            self.progress.progress += 1.0;
            return 0.0;
        }

        let x = self.progress.progress / self.settings.time as f32 * 100.0;
        x.floor()
    }
}

impl eframe::App for TickaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after_secs(1.0);

        if self.progress.last_ticker_update.elapsed() > Duration::from_millis(950) {
            self.update_progress();
            self.progress.last_ticker_update = Instant::now();
        };

        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.progress.progress.to_string());

            let progress_bar = ProgressBar::new(self.calculate_current_progress())
                .fill(Color32::BLUE)
                .show_percentage();

            ui.add(progress_bar);

            if ui.button("Go to Settings").clicked() {
                self.screen = Screen::SettingsScreen;
            }

            if let Screen::SettingsScreen = self.screen {
                ui.label("Settings");
                ui.add(egui::Slider::new(&mut self.settings.time, 0..=180).text("Duration"));
                if ui.button("Back to Main Screen").clicked() {
                    self.screen = Screen::MainScreen;
                }
            }
        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        // ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((200.0, 50.0)),
        ..eframe::NativeOptions::default()
    };

    let app = TickaApp::default();
    eframe::run_native("ticka", native_options, Box::new(|_cc| Ok(Box::new(app))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_calculation_half() {
        let mut app = TickaApp::default();
        app.progress.progress = 30.0;
        app.settings.time = 60;
        let progress = app.calculate_current_progress();
        assert_eq!(progress, 50.0);
    }
    #[test]
    fn test_progress_calculation_one_third() {
        let mut app = TickaApp::default();
        app.progress.progress = 20.0;
        app.settings.time = 60;
        let progress = app.calculate_current_progress();
        assert_eq!(progress, 33.0);
    }
}
