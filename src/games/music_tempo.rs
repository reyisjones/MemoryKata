use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MusicTempo {
    target_bpm: u16,
    answer: String,
    state: GameState,
}

impl MusicTempo {
    pub fn new() -> Self {
        let mut module = Self {
            target_bpm: 0,
            answer: String::new(),
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Match tempo markings to BPM".into(),
            },
        };
        module.reset();
        module
    }

    fn tempo_hint(bpm: u16) -> &'static str {
        match bpm {
            40..=60 => "Largo ♩",
            61..=76 => "Adagio ♫",
            77..=108 => "Andante ♪",
            109..=120 => "Moderato ♬",
            121..=168 => "Allegro ♯",
            _ => "Presto ♭",
        }
    }
}

impl Default for MusicTempo {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for MusicTempo {
    fn name(&self) -> &str {
        "Music Tempo"
    }

    fn description(&self) -> &str {
        &self.state.description
    }

    fn state(&self) -> &GameState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }

    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.target_bpm = rng.gen_range(40..=200);
        self.answer.clear();
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Music Tempo Trainer");
        ui.label(format!("Tempo hint: {}", Self::tempo_hint(self.target_bpm)));
        ui.label("Guess the BPM:");
        ui.add(egui::TextEdit::singleline(&mut self.answer));

        if ui.button("Check").clicked() {
            self.check_answer();
        }

        ui.label(format!("Score: {}", self.state.score));
        ui.label(format!("Attempts: {}", self.state.attempts));
    }

    fn check_answer(&mut self) {
        self.state.attempts += 1;
        if let Ok(value) = self.answer.trim().parse::<i32>() {
            let diff = (value - self.target_bpm as i32).abs();
            if diff <= 5 {
                self.state.score += 5;
            } else if diff <= 15 {
                self.state.score += 2;
            } else {
                self.state.score -= 1;
            }
        }
        self.reset();
    }
}
