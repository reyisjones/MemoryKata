use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PiChallenge {
    digits: Vec<char>,
    input: String,
    state: GameState,
}

impl PiChallenge {
    pub fn new() -> Self {
        Self {
            digits: "3141592653589793238462643383279".chars().collect(),
            input: String::new(),
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Memorize digits of π".into(),
            },
        }
    }

    fn current_sequence(&self, length: usize) -> String {
        self.digits.iter().take(length).collect()
    }
}

impl Default for PiChallenge {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for PiChallenge {
    fn name(&self) -> &str {
        "Π Challenge"
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
        self.input.clear();
        self.state.attempts = 0;
        self.state.score = 0;
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Π Challenge");
        ui.label("Enter the next digits of π: ∞");

        ui.add(egui::TextEdit::singleline(&mut self.input))
            .on_hover_text("Type digits like 3.14159...");

        if ui.button("Check").clicked() {
            self.check_answer();
        }

        ui.label(format!("Score: {}", self.state.score));
        ui.label(format!("Attempts: {}", self.state.attempts));
    }

    fn check_answer(&mut self) {
        let len = self.input.len();
        let target = self.current_sequence(len);
        self.state.attempts += 1;

        if self.input == target {
            self.state.score += len as i32;
        } else {
            self.state.score -= 1;
        }

        self.input.clear();
    }
}
