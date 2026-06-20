use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    command::CommandLine, cursor::Cursor, document::Document, mode::Mode, viewport::Viewport,
};

pub struct Editor {
    pub document: Document,
    pub cursor: Cursor,
    pub viewport: Viewport,
    pub mode: Mode,
    pub command: CommandLine,
    pub should_quit: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            document: Document::new(),
            cursor: Cursor::default(),
            viewport: Viewport::new(),
            mode: Mode::Normal,
            command: CommandLine::new(),
            should_quit: false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.mode {
            Mode::Normal => self.normal_mode(key),
            Mode::Command => self.command_mode(key.code),
            Mode::Insert => self.insert_mode(key),
            Mode::Visual => self.visual_mode(key),
        }
    }

    pub fn normal_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char(':') => self.mode = Mode::Command,
            KeyCode::Char('i') => self.mode = Mode::Insert,
            KeyCode::Char('v') => self.mode = Mode::Visual,
            KeyCode::Char('o') => {
                self.cursor.x = self.document.lines[self.cursor.y].len();
                self.insert_newline();
                self.mode = Mode::Insert;
            },
            KeyCode::Char('O') => {
                self.cursor.y -= 1;
                self.cursor.x = self.document.lines[self.cursor.y].len();
                self.insert_newline();
                self.mode = Mode::Insert;
            },
            KeyCode::Home => if key.modifiers == KeyModifiers::CONTROL { self.move_file_start() } else { self.move_line_start() },
            KeyCode::End => if key.modifiers == KeyModifiers::CONTROL { self.move_file_end() } else { self.move_line_end() },
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Up => self.cursor_up(),
            KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    pub fn command_mode(&mut self, code: KeyCode) {
        match code {
            KeyCode::Esc => {
                self.command.clear();
                self.mode = Mode::Normal;
            }
            KeyCode::Enter => {
                let command = self.command.stringify();
                match command[0] {
                    "q" => self.should_quit = true,
                    "w" => {
                        let _ = self.document.save_file(if command.len() > 1 { command[1] } else {&self.document.path});
                    },
                    "wq" => {
                        let _ = self.document.save_file(if command.len() > 1 { command[1] } else {&self.document.path});
                        self.should_quit = true;
                    },
                    "e" => {
                        if command.len() > 1 {
                            self.document.open_file(command[1]);
                        }
                    }
                    _ => {}
                }
                self.command.clear();
                self.mode = Mode::Normal;
            }
            KeyCode::Backspace => {
                self.command.pop();
            }
            KeyCode::Char(c) => {
                self.command.push(c);
            }
            _ => {}
        }
    }

    pub fn insert_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.mode = Mode::Normal,
            KeyCode::Char(c) => self.insert_char(c),
            KeyCode::Backspace => self.backspace(),
            KeyCode::Enter => self.insert_newline(),
            KeyCode::Home => if key.modifiers == KeyModifiers::CONTROL { self.move_file_start() } else { self.move_line_start() },
            KeyCode::End => if key.modifiers == KeyModifiers::CONTROL { self.move_file_end() } else { self.move_line_end() },
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Up => self.cursor_up(),
            KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    pub fn visual_mode(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Up => self.cursor_up(),
            KeyCode::Down => self.cursor_down(),
            _ => {}
        }
    }

    pub fn buffer_line(&self) -> String {
        self.document.path.clone()
    }

    pub fn status_line(&self) -> String {
        let mode: String;
        match self.mode {
            Mode::Normal => mode = "NORMAL".to_string(),
            Mode::Command => mode = format!(":{}", self.command.stringify().join(" ")),
            Mode::Insert => mode = "INSERT".to_string(),
            Mode::Visual => mode = "VISUAL".to_string(),
        }
        format!("{} | {} | {}:{}", mode, self.document.path, self.cursor.x, self.cursor.y)
    }

    pub fn insert_char(&mut self, c: char) {
        self.document.insert_char(self.cursor.x, self.cursor.y, c);
        self.cursor.x += 1;
        self.ensure_cursor_visible();
    }

    pub fn backspace(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
            self.document.delete_char(self.cursor.x, self.cursor.y);
        } else if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.document.lines[self.cursor.y].len();
            self.document.join_lines(self.cursor.y);
        }
        self.ensure_cursor_visible();
    }

    pub fn insert_newline(&mut self) {
        self.document.insert_newline(self.cursor.x, self.cursor.y);
        self.cursor.y += 1;
        self.cursor.x = 0;
        self.ensure_cursor_visible();
    }

    pub fn cursor_position(&self) -> (u16, u16) {
        (
            (self.cursor.x - self.viewport.col_offset) as u16,
            (self.cursor.y - self.viewport.row_offset) as u16,
        )
    }

    pub fn cursor_left(&mut self) {
        if self.cursor.x > 0 {
            self.cursor.x -= 1;
        }
        self.ensure_cursor_visible();
    }

    pub fn cursor_right(&mut self) {
        if self.cursor.x < self.document.lines[self.cursor.y].len() {
            self.cursor.x += 1;
        }
        self.ensure_cursor_visible();
    }

    pub fn cursor_up(&mut self) {
        if self.cursor.y > 0 {
            self.cursor.y -= 1;
            self.cursor.x = self.cursor.x.min(if self.cursor.y == 0 {
                0
            } else {
                self.document.lines[self.cursor.y - 1].len()
            });
        }
        self.ensure_cursor_visible();
    }

    pub fn cursor_down(&mut self) {
        if self.cursor.y < self.document.lines.len() - 1 {
            self.cursor.y += 1;
            self.cursor.x = self
                .cursor
                .x
                .min(if self.cursor.y == self.document.lines.len() - 1 {
                    0
                } else {
                    self.document.lines[self.cursor.y + 1].len()
                });
        }
        self.ensure_cursor_visible();
    }

    pub fn move_line_start(&mut self) {
        self.cursor.x = 0;
        self.ensure_cursor_visible();
    }

    pub fn move_line_end(&mut self) {
        self.cursor.x = self.document.lines[self.cursor.y].len();
        self.ensure_cursor_visible();
    }

    pub fn move_file_start(&mut self) {
        self.cursor.y = 0;
        self.cursor.x = 0;
        self.ensure_cursor_visible();
    }

    pub fn move_file_end(&mut self) {
        self.cursor.y = self.document.lines.len()-1;
        self.cursor.x = self.document.lines[self.cursor.y].len();
        self.ensure_cursor_visible();
    }
    
    pub fn ensure_cursor_visible(&mut self) {
        if self.cursor.y < self.viewport.row_offset {
            self.viewport.row_offset = self.cursor.y;
        }

        if self.cursor.y >= self.viewport.row_offset + self.viewport.height {
            self.viewport.row_offset = self.cursor.y - self.viewport.height + 1;
        }

        if self.cursor.x < self.viewport.col_offset {
            self.viewport.col_offset = self.cursor.x;
        }

        if self.cursor.x >= self.viewport.col_offset + self.viewport.width {
            self.viewport.col_offset = self.cursor.x - self.viewport.width + 1;
        }
    }

    pub fn visible_text(&self, height: usize) -> String {
        self.document
            .lines
            .iter()
            .skip(self.viewport.row_offset)
            .take(height)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn line_numbers(&self, height: usize) -> String {
        (self.viewport.row_offset..self.viewport.row_offset+height).map(|n| n.to_string()+"\n").collect()
    }
}