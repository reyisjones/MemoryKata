use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

const GREEK_LETTERS: &[(&str, &str)] = &[
    ("Α", "Alpha"),
    ("Β", "Beta"),
    ("Γ", "Gamma"),
    ("Δ", "Delta"),
    ("Ω", "Omega"),
    ("μ", "Mu"),
    ("π", "Pi"),
    ("λ", "Lambda"),
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GreekAlphabet {
    #[serde(skip, default)]
    prompt: Option<(&'static str, &'static str)>,
    answer: String,
    state: GameState,
}

impl GreekAlphabet {
    pub fn new() -> Self {
        let mut module = Self {
            prompt: None,
            answer: String::new(),
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Match Greek symbols to their names".into(),
            },
        };
        module.reset();
        module
    }
}

impl Default for GreekAlphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for GreekAlphabet {
    fn name(&self) -> &str {
        "Greek Alphabet"
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
        let mut rng = thread_rng();
        self.prompt = GREEK_LETTERS.choose(&mut rng).copied();
        self.answer.clear();
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Greek Alphabet");
        if let Some((symbol, _)) = self.prompt {
            ui.label(format!("Symbol: {}", symbol));
        }

        ui.label("Type the matching name:");
        ui.add(egui::TextEdit::singleline(&mut self.answer));

        if ui.button("Check").clicked() {
            self.check_answer();
        }

        ui.label(format!("Score: {}", self.state.score));
        ui.label(format!("Attempts: {}", self.state.attempts));
    }

    fn check_answer(&mut self) {
        self.state.attempts += 1;
        if let Some((_, name)) = self.prompt {
            if self.answer.trim().eq_ignore_ascii_case(name) {
                self.state.score += 2;
            } else {
                self.state.score -= 1;
            }
        }
        self.reset();
    }
}
