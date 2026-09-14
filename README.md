# 🎲 Rust Roller CLI Tool

A versatile, multi-lingual command-line tool for simulating complex dice roll mechanics used in tabletop RPGs and games of chance. `rust_roller` tracks session state, handles advanced rolling rules, and provides comprehensive analysis of success and failure across multiple rolls.

## ✨ Features
*   **Advanced Rolling Logic:** Simulates detailed success/failure outcomes, including critical successes and their associated bonus rolls.
*   **Session State Tracking:** Maintains a complete log of all roll outcomes (successes, failures, bonuses) for an end-of-session summary.
*   **Internationalization (i18n):** Supports multiple interface languages (e.g., English, French, German, Czech).
*   **State Management:** Core logic is built using pure functions operating on immutable state structures, ensuring reliable and predictable mechanics.

## 🚀 Getting Started

### Prerequisites
Ensure you have the Rust toolchain installed via `rustup`:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation & Setup
1.  **Clone the repository:**
    ```bash
    git clone [repository-url]
    cd rusted_roller
    ```
2.  **Build the project:**
    ```bash
    cargo build
    ```
3.  **Run the tool:**
    ```bash
    ./target/debug/rust_roller
    ```

## 🧭 Usage Guide

The tool operates via a guided, interactive command-line session:

### Step 1: Select Language
On the first run, the CLI will prompt you to select an interface locale (e.g., `en`, `fr`, `de`, `cs`).

### Step 2: Input Parameters
You will be prompted to enter the essential parameters for the simulation:
*   **Max Die Sides (N):** The maximum number of sides on the dice (e.g., `20` for d20).
*   **Target Value (T):** The minimum roll needed to count as a success.
*   **Number of Rolls:** How many times you want to perform a roll sequence.

### Step 3: Simulation & Results
The tool performs the rolling sequence in real-time, providing feedback for each roll.

**Core Mechanics Explained:**

*   **Success:** Rolling a value $\ge T$.
*   **Critical Success:** Rolling a natural maximum value (N). This triggers one or more **Bonus Rolls**, adding to the overall success count.
*   **Critical Failure:** Rolling a natural minimum value (`1`). This imposes a negative modifier, reducing the final success score.
*   **Final Calculation:** The session culminates in a summary that calculates the final adjusted success count:
    $$ \text{Final Successes} = \text{Regular Successes} - (\text{Critical Failures} \times \text{Modifier}) $$

### 🎨 Architecture Deep Dive
The engine's robust design separates presentation (I/O) from pure business logic.
*   **`src/main.rs`** contains the entire logic flow.
*   **State Management:** The `RollResults` struct handles the persistent state, ensuring all outcomes are tracked immutably throughout the session.
*   **Pure Functions:** The `process_roll()` function is separated, taking only the necessary inputs (roll value, roll parameters, random seed) and guaranteeing deterministic output, which is excellent for testing.

## 🛠 Development & Contribution
The project uses Rust and Cargo. For contribution, please follow standard Rust development practices. Use `cargo test` to run unit and integration tests.