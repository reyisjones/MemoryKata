use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

use super::{GameModule, GameState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CipherKind {
    Caesar,
    XOR,
    Base64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CryptoPuzzle {
    cipher: CipherKind,
    plaintext: String,
    ciphertext: String,
    answer: String,
    state: GameState,
}

impl CryptoPuzzle {
    pub fn new() -> Self {
        let mut module = Self {
            cipher: CipherKind::Caesar,
            plaintext: String::new(),
            ciphertext: String::new(),
            answer: String::new(),
            state: GameState {
                score: 0,
                attempts: 0,
                description: "Decode simple ciphers".into(),
            },
        };
        module.reset();
        module
    }

    fn generate_puzzle(&mut self) {
        let mut rng = rand::thread_rng();
        let words = vec!["HELLO", "MEMORY", "RUST", "KATA", "BRAIN"];
        self.plaintext = words.choose(&mut rng).unwrap_or(&"RUST").to_string();
        self.cipher = match rng.gen_range(0..3) {
            0 => CipherKind::Caesar,
            1 => CipherKind::XOR,
            _ => CipherKind::Base64,
        };

        self.ciphertext = match self.cipher {
            CipherKind::Caesar => self.plaintext
                .chars()
                .map(|c| (((c as u8 - b'A' + 3) % 26) + b'A') as char)
                .collect(),
            CipherKind::XOR => self
                .plaintext
                .bytes()
                .map(|b| (b ^ 0x2A) as char)
                .collect(),
            CipherKind::Base64 => STANDARD.encode(&self.plaintext),
        };
    }
}

impl Default for CryptoPuzzle {
    fn default() -> Self {
        Self::new()
    }
}

impl GameModule for CryptoPuzzle {
    fn name(&self) -> &str {
        "Crypto Puzzle"
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
        self.generate_puzzle();
        self.answer.clear();
    }

    fn update(&mut self, ui: &mut egui::Ui, _frame: &eframe::Frame) {
        ui.heading("Crypto Puzzle");
        let cipher_name = match self.cipher {
            CipherKind::Caesar => "Caesar shift +3",
            CipherKind::XOR => "XOR key 0x2A",
            CipherKind::Base64 => "Base64 ✱",
        };
        ui.label(format!("Cipher: {}", cipher_name));
        ui.label(format!("Ciphertext: {}", self.ciphertext));
        ui.add(egui::TextEdit::singleline(&mut self.answer));

        if ui.button("Decode").clicked() {
            self.check_answer();
        }

        ui.label(format!("Score: {}", self.state.score));
        ui.label(format!("Attempts: {}", self.state.attempts));
    }

    fn check_answer(&mut self) {
        self.state.attempts += 1;
        if self.answer.trim().eq_ignore_ascii_case(&self.plaintext) {
            self.state.score += 4;
        } else {
            self.state.score -= 1;
        }
        self.reset();
    }
}
