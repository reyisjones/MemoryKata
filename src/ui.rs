use eframe::egui::{self, Align, Layout, RichText, ViewportCommand};

use crate::games::{self, GameModule, GameState};
use crate::storage::{SaveData, Storage};

pub enum View {
    MainMenu,
    Game(usize),
    Settings,
    Scoreboard,
}

pub struct App {
    pub storage: Storage,
    pub data: SaveData,
    pub view: View,
    pub modules: Vec<Box<dyn GameModule + Send + Sync>>,
}

impl App {
    pub fn new(storage: Storage) -> Self {
        let modules: Vec<Box<dyn GameModule + Send + Sync>> = vec![
            Box::new(games::pi_challenge::PiChallenge::new()),
            Box::new(games::greek_alphabet::GreekAlphabet::new()),
            Box::new(games::music_tempo::MusicTempo::new()),
            Box::new(games::math_tricks::MathTricks::new()),
            Box::new(games::physics_formulas::PhysicsFormulas::new()),
            Box::new(games::periodic_table::PeriodicTable::new()),
            Box::new(games::crypto_puzzle::CryptoPuzzle::new()),
        ];

        let data = storage.load();
        let mut app = Self {
            storage,
            data,
            view: View::MainMenu,
            modules,
        };
        app.hydrate_scores();
        app
    }

    pub fn save(&mut self) {
        self.data.scores = self.modules.iter().map(|m| m.state().clone()).collect();
        self.storage.save(&self.data);
    }

    fn hydrate_scores(&mut self) {
        if self.data.scores.len() != self.modules.len() {
            self.data
                .scores
                .resize(self.modules.len(), GameState::default());
        }

        for (module, saved_state) in self.modules.iter_mut().zip(self.data.scores.iter()) {
            *module.state_mut() = saved_state.clone();
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.view {
            View::MainMenu => self.render_main_menu(ctx, frame),
            View::Settings => self.render_settings(ctx, frame),
            View::Scoreboard => self.render_scoreboard(ctx, frame),
            View::Game(index) => self.render_game(ctx, frame, index),
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.save();
    }
}

impl App {
    fn render_main_menu(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(RichText::new("MemoryKata 🧠").strong().size(32.0));
            ui.label("Sharpen your cognition with Unicode-powered mini-games ✨");

            ui.add_space(16.0);
            ui.heading("Select a kata:");

            for (index, module) in self.modules.iter().enumerate() {
                if ui.button(module.name()).clicked() {
                    self.view = View::Game(index);
                }
                ui.label(RichText::new(module.description()).italics());
                ui.separator();
            }

            ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                if ui.button("⚙️ Settings").clicked() {
                    self.view = View::Settings;
                }
                if ui.button("🏆 Scoreboard").clicked() {
                    self.view = View::Scoreboard;
                }
                if ui.button("Exit").clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                }
            });
        });
    }

    fn render_settings(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("⚙️ Settings");
            ui.checkbox(&mut self.data.settings.audio_enabled, "Enable sound effects ♫");
            ui.horizontal(|ui| {
                ui.label("Theme:");
                ui.selectable_value(&mut self.data.settings.theme, crate::storage::Theme::Light, "☀️ Light");
                ui.selectable_value(&mut self.data.settings.theme, crate::storage::Theme::Dark, "🌙 Dark");
            });

            ui.separator();
            if ui.button("Back").clicked() {
                self.view = View::MainMenu;
            }
        });
    }

    fn render_scoreboard(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🏆 Scoreboard");
            ui.label("Track your kata mastery:");

            for module in &self.modules {
                let state = module.state();
                ui.label(format!("{} → Score: {} | Attempts: {}", module.name(), state.score, state.attempts));
                ui.label(RichText::new(module.description()).italics());
                ui.separator();
            }

            if ui.button("Back").clicked() {
                self.view = View::MainMenu;
            }
        });
    }

    fn render_game(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame, index: usize) {
        if let Some(module) = self.modules.get_mut(index) {
            let mut back_to_menu = false;
            let mut reset_requested = false;

            {
                egui::CentralPanel::default().show(ctx, |ui| {
                    module.update(ui, frame);
                    ui.separator();
                    if ui.button("Return to menu ↩").clicked() {
                        back_to_menu = true;
                    }
                    if ui.button("Reset ♻️").clicked() {
                        reset_requested = true;
                    }
                });
            }

            if reset_requested {
                module.reset();
            }
            if back_to_menu {
                self.view = View::MainMenu;
            }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("Module not found");
                if ui.button("Back").clicked() {
                    self.view = View::MainMenu;
                }
            });
        }
    }
}
