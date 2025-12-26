use ratatui::style::Stylize;
use ratatui::text::Span;

use crossterm::event::KeyCode;

pub struct HotKey {
    text: String,
    pub hotkey: KeyCode,
}

impl HotKey {
    pub fn new(text: &str, hotkey: KeyCode) -> HotKey {
        let mut text = text.to_string();
        text.push(' ');
        Self { text, hotkey }
    }
    pub fn get_styled(&self) -> Vec<Span<'_>> {
        let mut hotkey_text = String::new();
        match self.hotkey {
            KeyCode::Char(c) => {
                hotkey_text.push('^');
                hotkey_text.push(c.to_ascii_uppercase());
            }
            _ => {
                hotkey_text = self.hotkey.to_string();
            }
        }
        hotkey_text.push(' ');

        vec![self.text.as_str().into(), hotkey_text.blue().bold()]
    }
}
