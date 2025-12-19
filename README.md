# FastType Rust

A minimalist, terminal-based typing tutor (TUI) written in Rust.
Designed for a distraction-free experience with typewriter scrolling and focus mode.

## Features

- **Focus Mode:** The active line is always centered vertically and horizontally.
- **Real-time Feedback:** Instant visual feedback for correct (green) and incorrect (red) characters.
- **Auto-Save:** Your progress is automatically saved to a `.save` file.

## Known Bugs

It sometimes can happen, that zou write below the gray text, not on it. Simply continue
to write to the next line, exit and restart the application. It should fix itself.

## Installation & Usage

You need a working Rust environment (Cargo) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/KJCats247/fasttype-rust.git
   cd fasttype-rust
   cargo run -- file.txt
   ```

## Built With

- Ratatui - Library for cooking up Terminal User Interfaces
- Crossterm - Cross-platform terminal handling
