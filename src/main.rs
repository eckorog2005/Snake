mod game;

use std::io;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    widgets::Paragraph,
};

fn main() -> io::Result<()> {
    // init() also installs a panic hook that restores the terminal, so a crash
    // doesn't leave the shell in raw mode.
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            frame.render_widget(Paragraph::new("Snake: press q to quit").centered(), frame.area());
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
                _ => {}
            }
        }
    }
}
