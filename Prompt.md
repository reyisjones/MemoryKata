## 🧠 Rust Memory Games App — Prompt

**Goal:**
Create a new **desktop application in Rust** called `MemoryKata` that helps users **improve memory and cognition** through simple puzzle-based mini-games. The app should have a **minimal, responsive UI** using Rust UI libraries (e.g., **egui**, **iced**, or **druid**) and display fonts and symbols from the **character map** (Unicode, Greek letters, music symbols, math operators, etc.).

---

### 🎯 Functional Requirements

1. **Main Menu**

   * Home screen with category buttons:

     * 🧩 *Pattern Memory* (color, number, and symbol sequences)
     * 🔢 *Math Tricks* (operations memorization and recall)
     * 🧮 *Pi Challenge* (memorize digits of π)
     * 🇬🇷 *Greek Alphabet* (symbol recall and matching)
     * 🎵 *Music Tempo Trainer* (BPM and notation memory)
     * ⚛️ *Physics Formulas* (formula recall quiz)
     * 🧬 *Periodic Table* (element name/symbol association)
     * 🔐 *Crypto Puzzle* (simple ciphers like Caesar, XOR, base64)
   * Settings and Scoreboard sections.

2. **Gameplay**

   * Each mini-game should use a modular structure (trait `GameModule` with methods like `init`, `render`, `check_answer`).
   * Provide randomized rounds for replayability.
   * Store user progress locally in a small JSON or SQLite file.

3. **UI / Design**

   * Built with **egui** (preferred) or **iced**.
   * Use Unicode and system fonts from the **character map**:

     * Greek: `Α β γ Δ Ω`
     * Music: `♩ ♪ ♬ ♭ ♯`
     * Math: `∑ ∆ π ≈ ∞`
     * Chemistry: `H₂O`, `NaCl`
   * Include keyboard and mouse input interactions.

4. **Tech Stack**

   * Language: **Rust**
   * UI Framework: **egui** or **iced**
   * Data Storage: **serde_json** or **rusqlite**
   * Build System: **cargo**
   * OS: Windows/macOS/Linux
   * (Optional) Sound effects using `rodio`

5. **Project Structure**

   ```
   memorykata/
   ├── src/
   │   ├── main.rs
   │   ├── ui.rs
   │   ├── games/
   │   │   ├── mod.rs
   │   │   ├── pi_challenge.rs
   │   │   ├── greek_alphabet.rs
   │   │   ├── math_tricks.rs
   │   │   ├── music_tempo.rs
   │   │   ├── physics_formulas.rs
   │   │   ├── periodic_table.rs
   │   │   └── crypto_puzzle.rs
   │   └── storage.rs
   ├── Cargo.toml
   └── assets/
       ├── fonts/
       └── sounds/
   ```

6. **Features to Implement Later**

   * Local high-score tracking
   * Daily challenge rotation
   * Export/import user progress
   * Theming (light/dark mode)

---
 
 
 
