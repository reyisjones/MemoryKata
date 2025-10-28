use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MathTricks {
    expression: String,
    answer: String,
    expected: i32,
    state: GameState,
}

impl MathTricks {
    pub fn new() -> Self {
        let mut module = Self {
            expression: String::new(),
            answer: String::new(),
            expected: 0,
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Recall math tricks quickly".into(),
            },
        };
        module.reset();
        module
    }

    fn generate_expression() -> (String, i32) {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(2..12);
        let b = rng.gen_range(2..12);
        let c = rng.gen_range(1..6);
        let exprs = vec![
            (format!("({} × {}) + {}", a, b, c), a * b + c),
            (format!("{}² + {}", a, c), a * a + c),
            (format!("{}! / {}", a, b), (2..=a).product::<i32>() / b.max(1)),
        ];
        exprs.choose(&mut rng).cloned().unwrap_or(exprs[0].clone())
    }
}

impl Default for MathTricks {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for MathTricks {
    fn name(&self) -> &str {
        "Math Tricks"
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
        let (expr, expected) = Self::generate_expression();
        self.expression = expr;
        self.expected = expected;
        self.answer.clear();
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Math Tricks");
        ui.label("Solve quickly and input the result:");
        ui.label(format!("Expression: {}", self.expression));
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
            if value == self.expected {
                self.state.score += 3;
            } else {
                self.state.score -= 1;
            }
        }
        self.reset();
    }
}
