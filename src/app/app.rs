use crate::app::ffmpeg_manager::FfmpegManager;
use crate::app::hotkey::HotKey;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::Constraint::{Fill, Length, Min};
use ratatui::style::Stylize;
use ratatui::widgets::{Gauge, List, ListItem, Paragraph, StatefulWidget};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Layout,
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};
use std::io;

pub struct App {
    exit: bool,
    hotkeys: Vec<HotKey>,
    ffmpeg_manager: FfmpegManager,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            hotkeys: vec![
                HotKey::new("Open file", KeyCode::Char('o')),
                HotKey::new("Close app", KeyCode::Char('c')),
            ],
            ffmpeg_manager: FfmpegManager::default(),
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                self.draw(frame);
            })?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
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
            KeyCode::Up => self.ffmpeg_manager.selected_file.select_previous(),
            KeyCode::Down => self.ffmpeg_manager.selected_file.select_next(),

            _ => {}
        }
    }
    fn render_settings(&mut self, area: Rect, buf: &mut Buffer) {
        let [sources_rect, settings_rect, files_rect] =
            Layout::horizontal([Fill(2), Length(25), Fill(1)]).areas(area);
        let sources_block = Block::bordered()
            .title(Line::from("Sources").centered())
            .border_set(border::ROUNDED);
        let settings_block = Block::bordered()
            .title(Line::from("Settings").centered())
            .border_set(border::ROUNDED);

        self.render_files_list(files_rect, buf);

        sources_block.render(sources_rect, buf);
        settings_block.render(settings_rect, buf);
    }
    fn render_files_list(&mut self, area: Rect, buf: &mut Buffer) {
        let files_block = Block::bordered()
            .title(Line::from("Files").centered())
            .border_set(border::ROUNDED);

        let items: Vec<ListItem> = self
            .ffmpeg_manager
            .files
            .iter()
            .map(|file| ListItem::from(file.to_string()))
            .collect();
        let list = List::new(items).block(files_block).highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut self.ffmpeg_manager.selected_file);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Film Compressor ".bold());
        let mut hotkeys = vec![" ".into()];
        self.hotkeys.iter().for_each(|hotkey| {
            hotkeys.extend(hotkey.get_styled());
        });
        let instructions = Line::from(hotkeys);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        let [main_page, command, progress] =
            Layout::vertical([Min(0), Length(3), Length(3)]).areas(block.inner(area));
        let command_block = Block::bordered()
            .border_set(border::ROUNDED)
            .title(Line::from(" Command ").centered());
        let progress_block = Block::bordered()
            .border_set(border::ROUNDED)
            .title(Line::from(" ffmpeg progress ").centered());
        let ffmpeg_command = Paragraph::new("ffmpeg ...").block(command_block);
        let progress_bar = Gauge::default().block(progress_block).ratio(0.5);

        block.render(area, buf);

        ffmpeg_command.render(command, buf);
        progress_bar.render(progress, buf);

        self.render_settings(main_page, buf);
    }
}
