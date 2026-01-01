use ratatui::style::Stylize;
use ratatui::text::Span;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub struct HotKey {
    pub text: String,
    pub key_event: KeyEvent,
}

impl HotKey {
    pub fn get_styled(&self) -> Vec<Span<'_>> {
        let mut hotkey_text = String::from(" ");
        match self.key_event.code {
            KeyCode::Char(c) => {
                if self.key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    hotkey_text.push('^');
                }
                hotkey_text.push(c.to_ascii_uppercase());
            }
            _ => {
                hotkey_text += &self.key_event.code.to_string();
            }
        }
        hotkey_text.push(' ');

        vec![self.text.as_str().into(), hotkey_text.blue().bold()]
    }
}
