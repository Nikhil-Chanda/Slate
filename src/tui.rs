use std::io;

use crossterm::{event::{self, Event, KeyEventKind}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Frame, Terminal, backend::CrosstermBackend, layout::{Constraint, Layout}, style::{Color, Style}, widgets::Paragraph};

use crate::editor::{Editor};

struct TuiGuard;

impl Drop for TuiGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = execute!(stdout, LeaveAlternateScreen);
    }
}

pub fn run(mut editor: &mut Editor) -> color_eyre::Result<()> {
    enable_raw_mode()?;
    let _guard = TuiGuard;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    loop {
        terminal.draw(|f| render(f, &mut editor))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                editor.handle_key(key);
            }
        }
        if editor.should_quit {
            break Ok(());
        }
    }
}

pub fn render(frame: &mut Frame, editor: &mut Editor) {
    let area = frame.area();

    let [buffer_area, editor_set_area, status_area] = Layout::vertical([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)]).areas(area);
    let [line_number_area, line_editor_cushion, editor_area] = Layout::horizontal([Constraint::Length(3), Constraint::Length(1), Constraint::Min(1)]).areas(editor_set_area);
    
    frame.render_widget(Paragraph::new(editor.buffer_line()).style(Style::default().bg(Color::Rgb(133, 193, 255))), buffer_area);
    frame.render_widget(Paragraph::new(editor.line_numbers(editor_area.height as usize)).right_aligned().style(Style::default().bg(Color::Rgb(40, 44, 52))), line_number_area);
    frame.render_widget(Paragraph::new(" ").style(Style::default().bg(Color::Rgb(40, 44, 52))), line_editor_cushion);
    frame.render_widget(Paragraph::new(editor.visible_text(editor_area.height as usize)).style(Style::default().bg(Color::Rgb(40, 44, 52))), editor_area);
    frame.render_widget(Paragraph::new(editor.status_line()).style(Style::default().bg(Color::Rgb(133, 193, 255))), status_area);

    let (x, y) = editor.cursor_position();
    editor.viewport.height = editor_area.height as usize;
    editor.viewport.width = editor_area.width as usize;
    frame.set_cursor_position((editor_area.x+x, editor_area.y+y));
}