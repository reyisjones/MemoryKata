pub mod pi_challenge;
pub mod greek_alphabet;
pub mod music_tempo;
pub mod math_tricks;
pub mod physics_formulas;
pub mod periodic_table;
pub mod crypto_puzzle;

use eframe::egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub score: i32,
    pub attempts: i32,
    pub description: String,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            attempts: 0,
            description: String::new(),
        }
    }
}

pub trait GameModule {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn state(&self) -> &GameState;
    fn state_mut(&mut self) -> &mut GameState;
    fn reset(&mut self);
    fn update(&mut self, ui: &mut Ui, frame: &eframe::Frame);
    fn check_answer(&mut self);
}
