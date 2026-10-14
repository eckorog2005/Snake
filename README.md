# Snake

A terminal snake game written in Rust with [ratatui](https://ratatui.rs).

## Setup

Install Rust via [rustup](https://rustup.rs) (or `brew install rustup && rustup default stable`).

## Play

```sh
cargo run --release
```

| Key | Action |
|---|---|
| Arrow keys / WASD | Turn |
| `r` | Restart after game over |
| `q` / `Esc` / `Ctrl-C` | Quit |

The terminal must be at least 42×17.

## Develop

```sh
cargo test     # game logic unit tests
cargo clippy   # lints
cargo fmt      # formatting
```

- `src/game.rs`: game rules (movement, growth, collisions, food), with no terminal code
- `src/ui.rs`: draws a `Game` with ratatui
- `src/main.rs`: terminal setup, input handling and the fixed-tick game loop
