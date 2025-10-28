use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

const FORMULAS: &[(&str, &str)] = &[
    ("F = m · a", "Newton's Second Law"),
    ("E = m · c²", "Mass–energy equivalence"),
    ("ΔE = h · ν", "Planck relation"),
    ("V = I · R", "Ohm's Law"),
    ("pV = nRT", "Ideal Gas Law"),
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PhysicsFormulas {
    #[serde(skip, default)]
    prompt: Option<(&'static str, &'static str)>,
    answer: String,
    state: GameState,
}

impl PhysicsFormulas {
    pub fn new() -> Self {
        let mut module = Self {
            prompt: None,
            answer: String::new(),
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Recall famous physics formulas".into(),
            },
        };
        module.reset();
        module
    }
}

impl Default for PhysicsFormulas {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for PhysicsFormulas {
    fn name(&self) -> &str {
        "Physics Formulas"
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
        self.prompt = FORMULAS.choose(&mut rng).copied();
        self.answer.clear();
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Physics Formulas");
        if let Some((formula, _)) = self.prompt {
            ui.label(format!("Formula: {}", formula));
        }
        ui.label("Name this formula:");
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
