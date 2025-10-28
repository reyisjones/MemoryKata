use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

const ELEMENTS: &[(&str, &str)] = &[
    ("H", "Hydrogen"),
    ("He", "Helium"),
    ("Li", "Lithium"),
    ("C", "Carbon"),
    ("Na", "Sodium"),
    ("Cl", "Chlorine"),
    ("Fe", "Iron"),
    ("Au", "Gold"),
];

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeriodicTable {
    #[serde(skip, default)]
    prompt: Option<(&'static str, &'static str)>,
    answer: String,
    state: GameState,
}

impl PeriodicTable {
    pub fn new() -> Self {
        let mut module = Self {
            prompt: None,
            answer: String::new(),
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Match element symbols to names".into(),
            },
        };
        module.reset();
        module
    }
}

impl Default for PeriodicTable {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for PeriodicTable {
    fn name(&self) -> &str {
        "Periodic Table"
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
        self.prompt = ELEMENTS.choose(&mut rng).copied();
        self.answer.clear();
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Periodic Table");
        if let Some((symbol, _)) = self.prompt {
            ui.label(format!("Symbol: {}", symbol));
        }
        ui.label("Name the element:");
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
