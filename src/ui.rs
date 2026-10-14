use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
};

use crate::game::{Game, Point, Status};

/// Terminal cells are roughly twice as tall as they are wide, so each board
/// cell is drawn two characters wide to look square.
const CELL_WIDTH: u16 = 2;

pub fn draw(frame: &mut Frame, game: &Game) {
    let board_width = game.width() as u16 * CELL_WIDTH + 2;
    let board_height = game.height() as u16 + 2;

    let screen = frame.area();
    if screen.width < board_width || screen.height < board_height {
        let message = format!("Terminal too small: need {board_width}x{board_height}");
        frame.render_widget(Paragraph::new(message).centered(), screen);
        return;
    }

    let area = centered(screen, board_width, board_height);
    let block = Block::bordered()
        .title(Line::from(" Snake ").bold().centered())
        .title_bottom(Line::from(format!(" Score: {} ", game.score())).centered());
    frame.render_widget(Paragraph::new(board_lines(game)).block(block), area);

    match game.status() {
        Status::Playing => {}
        Status::GameOver => draw_popup(frame, area, "Game over", Color::Red),
        Status::Won => draw_popup(frame, area, "You win!", Color::Green),
    }
}

fn board_lines(game: &Game) -> Vec<Line<'static>> {
    let mut cells = vec![vec![Span::raw("  "); game.width() as usize]; game.height() as usize];
    let mut set = |p: Point, span: Span<'static>| cells[p.y as usize][p.x as usize] = span;

    if let Some(food) = game.food() {
        set(food, Span::styled("()", Style::new().fg(Color::Red)));
    }
    for segment in game.snake() {
        set(segment, Span::styled("██", Style::new().fg(Color::Green)));
    }
    set(
        game.head(),
        Span::styled("██", Style::new().fg(Color::LightGreen)),
    );

    cells.into_iter().map(Line::from).collect()
}

fn draw_popup(frame: &mut Frame, board: Rect, title: &str, color: Color) {
    let text = vec![
        Line::from(title).bold().fg(color),
        Line::from(""),
        Line::from("r restart  q quit"),
    ];
    let area = centered(board, 22, text.len() as u16 + 2);
    frame.render_widget(Clear, area);
    frame.render_widget(
        Paragraph::new(text).centered().block(Block::bordered()),
        area,
    );
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let [area] = Layout::horizontal([Constraint::Length(width)])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([Constraint::Length(height)])
        .flex(Flex::Center)
        .areas(area);
    area
}
