mod game;
mod ui;

use std::{
    io,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
};

use game::{Direction, Game, Status};

const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 15;
const TICK: Duration = Duration::from_millis(120);

fn main() -> io::Result<()> {
    // init() also installs a panic hook that restores the terminal, so a crash
    // doesn't leave the shell in raw mode.
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut game = Game::new(BOARD_WIDTH, BOARD_HEIGHT);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &game))?;

        // Wait for input only until the next tick is due, so the snake keeps
        // moving even when no keys are pressed.
        let timeout = TICK.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                KeyCode::Char('r') if game.status() != Status::Playing => {
                    game = Game::new(BOARD_WIDTH, BOARD_HEIGHT);
                    last_tick = Instant::now();
                }
                KeyCode::Up | KeyCode::Char('w') => game.turn(Direction::Up),
                KeyCode::Down | KeyCode::Char('s') => game.turn(Direction::Down),
                KeyCode::Left | KeyCode::Char('a') => game.turn(Direction::Left),
                KeyCode::Right | KeyCode::Char('d') => game.turn(Direction::Right),
                _ => {}
            }
        }

        if last_tick.elapsed() >= TICK {
            game.tick();
            last_tick = Instant::now();
        }
    }
}
