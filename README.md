# MemoryKata 🧠

MemoryKata is a cross-platform Rust desktop application that sharpens memory and cognition through Unicode-powered mini-games.

## ✨ Features

- Modular mini-games: Π Challenge, Greek Alphabet, Music Tempo, Math Tricks, Physics Formulas, Periodic Table, Crypto Puzzle
- Responsive UI built with [egui](https://github.com/emilk/egui)
- Unicode-rich interface using Greek letters (Α β γ Δ Ω), music symbols (♩ ♪ ♬ ♭ ♯), math operators (∑ ∆ π ≈ ∞), and more
- Local storage of progress using `serde_json`
- Settings and Scoreboard views

## 🗂 Project Structure

```
memorykata/
├── assets/
│   ├── fonts/
│   └── sounds/
├── Cargo.toml
├── README.md
└── src/
    ├── games/
    │   ├── crypto_puzzle.rs
    │   ├── greek_alphabet.rs
    │   ├── math_tricks.rs
    │   ├── mod.rs
    │   ├── music_tempo.rs
    │   ├── periodic_table.rs
    │   ├── physics_formulas.rs
    │   └── pi_challenge.rs
    ├── main.rs
    ├── storage.rs
    └── ui.rs
```

## 🚀 Getting Started

```bash
# Install Rust toolchain
rustup install stable

# Run the application
cargo run
```

## 🧩 Gameplay Modules

| Module | Focus | Symbols |
| ------ | ----- | ------- |
| Π Challenge | Memorize digits of π | ∞ π |
| Greek Alphabet | Symbol/name recall | Α β γ Δ Ω |
| Music Tempo | BPM recognition | ♩ ♪ ♬ ♭ ♯ |
| Math Tricks | Rapid mental math | ∑ ∆ ≈ |
| Physics Formulas | Formula recall | F = m·a |
| Periodic Table | Element associations | H₂O NaCl |
| Crypto Puzzle | Decode simple ciphers | XOR, Base64 |

## 🧱 Roadmap

- Persistent high scores
- Daily challenges
- Import/export progress
- Theme customization
- Sound FX with `rodio`

## 📝 License

MIT License © 2025 Your Name
