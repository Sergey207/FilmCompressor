use crate::app::hotkey::HotKey;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Stylize;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};
use std::io;

pub struct App {
    exit: bool,
    hotkeys: Vec<HotKey>,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            hotkeys: vec![HotKey::new("Open file", KeyCode::Char('o'))],
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('o') => {}
            KeyCode::Char('q') | KeyCode::Char('c') | KeyCode::Esc => {
                self.exit = true;
            }
            _ => {}
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Film Compressor ".bold());
        let mut hotkeys = vec![" ".into()];
        self.hotkeys.iter().for_each(|hotkey| {
            hotkeys.extend(hotkey.get_styled());
        });
        hotkeys.push(" ".into());
        let instructions = Line::from(hotkeys);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        block.render(area, buf);
    }
}
