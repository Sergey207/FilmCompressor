use crate::app::ffmpeg_manager::{
    AudioCodec, FfmpegManager, PixelFormat, SubtitleCodec, VideoCodec,
};
use crate::app::hotkey::HotKey;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use ratatui::layout::Constraint::{Fill, Length, Min};
use ratatui::style::Stylize;
use ratatui::widgets::{List, ListItem, ListState, Paragraph, StatefulWidget, Wrap};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Layout,
    layout::Rect,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};
use std::cmp::min;
use std::fs::create_dir;
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};
use strum::IntoEnumIterator;

pub struct App {
    exit: bool,
    hotkeys: Vec<HotKey>,
    ffmpeg_manager: FfmpegManager,
    selections: [ListState; 3],
    selected_compress_setting: ListState,
    editing_string: Option<String>,
}

impl App {
    pub fn new() -> Self {
        let mut new_app = Self {
            exit: false,
            hotkeys: Vec::new(),
            ffmpeg_manager: FfmpegManager::default(),
            selections: [ListState::default(); 3],
            selected_compress_setting: ListState::default(),
            editing_string: None,
        };
        new_app.update_hotkeys();
        let args = env::args().collect::<Vec<_>>();
        match args.len() {
            1 => new_app
                .ffmpeg_manager
                .add_folder(env::current_dir().unwrap()),
            _ => args[1..].iter().for_each(|arg| {
                new_app.ffmpeg_manager.add_path(PathBuf::from(arg));
            }),
        }
        new_app
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

    fn update_hotkeys(&mut self) {
        let mut result = vec![
            HotKey {
                text: "Run".to_string(),
                key_event: KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::empty(),
                },
            },
            HotKey {
                text: "Close app".to_string(),
                key_event: KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::empty(),
                },
            },
        ];
        if let Some(selection) = self.get_selected() {
            match selection {
                0 => {
                    result.push(HotKey {
                        text: "Toggle enabled".to_string(),
                        key_event: KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: KeyModifiers::empty(),
                            kind: KeyEventKind::Press,
                            state: KeyEventState::empty(),
                        },
                    });
                    result.push(HotKey {
                        text: "Toggle default".to_string(),
                        key_event: KeyEvent {
                            code: KeyCode::Char('d'),
                            modifiers: KeyModifiers::CONTROL,
                            kind: KeyEventKind::Press,
                            state: KeyEventState::empty(),
                        },
                    });
                }
                1 => {
                    result.push(HotKey {
                        text: "Change".to_string(),
                        key_event: KeyEvent {
                            code: KeyCode::Enter,
                            modifiers: KeyModifiers::empty(),
                            kind: KeyEventKind::Press,
                            state: KeyEventState::empty(),
                        },
                    });
                    if self.selected_compress_setting.selected().is_some()
                        || self.editing_string.is_some()
                    {
                        result.push(HotKey {
                            text: "Exit".to_string(),
                            key_event: KeyEvent {
                                code: KeyCode::Esc,
                                modifiers: KeyModifiers::empty(),
                                kind: KeyEventKind::Press,
                                state: KeyEventState::empty(),
                            },
                        })
                    }
                }
                2 => result.push(HotKey {
                    text: "Delete file from list".to_string(),
                    key_event: KeyEvent {
                        code: KeyCode::Delete,
                        modifiers: KeyModifiers::empty(),
                        kind: KeyEventKind::Press,
                        state: KeyEventState::empty(),
                    },
                }),
                _ => unreachable!(),
            }
        }
        self.hotkeys = result;
    }

    /// Returns inde of selected list (0/1/2) or None
    pub fn get_selected(&self) -> Option<usize> {
        for i in 0..3 {
            if self.selections[i].selected().is_some() {
                return Some(i);
            }
        }
        None
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
            KeyCode::Char('r') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.run_compressing();
                }
            }
            KeyCode::Char('q') | KeyCode::Char('c') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    self.exit = true;
                }
            }
            KeyCode::Esc => {
                if self.selected_compress_setting.selected().is_some() {
                    self.selected_compress_setting.select(None);
                } else if self.editing_string.is_some() {
                    self.editing_string = None;
                } else {
                    self.exit = true;
                }
            }
            KeyCode::Up | KeyCode::Down => {
                if let Some(selected) = self.get_selected() {
                    if self.selected_compress_setting.selected().is_some() {
                        match key_event.code {
                            KeyCode::Up => self.selected_compress_setting.select_previous(),
                            KeyCode::Down => self.selected_compress_setting.select_next(),
                            _ => {}
                        }
                    } else {
                        match key_event.code {
                            KeyCode::Up => self.selections[selected].select_previous(),
                            KeyCode::Down => self.selections[selected].select_next(),
                            _ => {}
                        }
                    }
                } else {
                    self.selections[0].select_first();
                }
            }
            KeyCode::Left | KeyCode::Right => {
                let selected = self.get_selected();
                if let Some(mut selected) = selected {
                    self.selections[selected].select(None);
                    if key_event.code == KeyCode::Left {
                        selected = selected.saturating_sub(1);
                    } else {
                        selected = min(selected + 1, 2);
                    }
                    self.selections[selected].select_first();
                } else {
                    self.selections[0].select_first();
                }
                self.selected_compress_setting.select(None);
                self.editing_string = None;
            }
            KeyCode::Enter => {
                if let Some(selection) = self.get_selected() {
                    match selection {
                        0 => {
                            let selection = self.selections[0].selected().unwrap();
                            self.ffmpeg_manager.stream_settings[selection].toggle_enabled();
                        }
                        1 => {
                            let selection = self.selections[1].selected().unwrap();
                            if let Some(compress_setting_selection) =
                                self.selected_compress_setting.selected()
                            // Codec|pix_fmt comboBox
                            {
                                match selection {
                                    0 => {
                                        self.ffmpeg_manager.compress_settings.video_codec =
                                            VideoCodec::iter().collect::<Vec<VideoCodec>>()
                                                [compress_setting_selection]
                                                .clone()
                                    }
                                    1 => {
                                        self.ffmpeg_manager.compress_settings.pixel_format =
                                            PixelFormat::iter().collect::<Vec<PixelFormat>>()
                                                [compress_setting_selection]
                                                .clone()
                                    }
                                    2 => {
                                        self.ffmpeg_manager.compress_settings.audio_codec =
                                            AudioCodec::iter().collect::<Vec<AudioCodec>>()
                                                [compress_setting_selection]
                                                .clone()
                                    }
                                    3 => {
                                        self.ffmpeg_manager.compress_settings.subtitle_codec =
                                            SubtitleCodec::iter().collect::<Vec<SubtitleCodec>>()
                                                [compress_setting_selection]
                                                .clone()
                                    }
                                    _ => unreachable!(),
                                }
                                self.selected_compress_setting.select(None);
                            } else if self.editing_string.is_some() { // bitrate|scale|other input
                            } else {
                                match selection {
                                    0..4 => self.selected_compress_setting.select_first(),
                                    4..8 => self.editing_string = Some(String::new()),
                                    _ => unreachable!(),
                                }
                            }
                        }
                        2 => {}
                        _ => unreachable!(),
                    }
                }
            }
            KeyCode::Char('d') => {
                if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    if let Some(selection) = self.selections[0].selected() {
                        self.ffmpeg_manager.toggle_default(selection);
                    }
                }
            }
            KeyCode::Delete => {
                if let Some(selection) = self.selections[2].selected() {
                    self.ffmpeg_manager.input_files.remove(selection);
                    self.ffmpeg_manager.update_stream_settings();
                }
            }
            _ => {}
        }
        self.update_hotkeys();
    }

    fn run_compressing(&mut self) {
        let mut index = 0usize;
        let mut output_folder;
        loop {
            output_folder = env::current_dir().unwrap().clone();
            if index == 0 {
                output_folder.push("output");
            } else {
                output_folder.push(format!("output ({})", index));
            }
            if !output_folder.exists() {
                break;
            }
            index += 1;
        }
        create_dir(&output_folder).unwrap();
        self.ffmpeg_manager
            .input_files
            .iter()
            .for_each(|input_file| {
                let mut output_file = output_folder.clone();
                output_file.push(input_file.path.file_name().unwrap().to_str().unwrap());
                let ffmpeg_command = self.ffmpeg_manager.get_command(input_file, &output_file);
                Command::new("ffmpeg")
                    .args(&ffmpeg_command)
                    .output()
                    .unwrap();
            });
        self.exit = true;
    }

    fn render_settings(&mut self, area: Rect, buf: &mut Buffer) {
        let [sources_rect, compress_settings_rect, files_rect] =
            Layout::horizontal([Fill(2), Length(30), Fill(1)]).areas(area);

        self.render_sources_list(sources_rect, buf);
        self.render_compress_settings_list(compress_settings_rect, buf);
        self.render_files_list(files_rect, buf);
    }

    fn render_sources_list(&mut self, area: Rect, buf: &mut Buffer) {
        let sources_block = Block::bordered()
            .title(Line::from(" Sources ").centered())
            .border_set(border::ROUNDED);
        let items: Vec<ListItem> = self
            .ffmpeg_manager
            .stream_settings
            .iter()
            .map(|source| ListItem::from(source.to_string()))
            .collect();
        let list = List::new(items).block(sources_block).highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut self.selections[0]);
    }

    fn render_compress_settings_list(&mut self, area: Rect, buf: &mut Buffer) {
        let settings_block = Block::bordered()
            .title(Line::from(" Settings ").centered())
            .border_set(border::ROUNDED);
        let mut items = vec![];
        if self.selected_compress_setting.selected().is_some() {
            items = match self.selections[1].selected().unwrap() {
                0 => VideoCodec::iter()
                    .map(|codec| {
                        let mut result = String::new();
                        result += if codec == self.ffmpeg_manager.compress_settings.video_codec {
                            "[X] "
                        } else {
                            "[ ] "
                        };
                        result += &codec.to_string();
                        ListItem::new(result)
                    })
                    .collect(),
                1 => PixelFormat::iter()
                    .map(|codec| {
                        let mut result = String::new();
                        result += if codec == self.ffmpeg_manager.compress_settings.pixel_format {
                            "[X] "
                        } else {
                            "[ ] "
                        };
                        result += &codec.to_string();
                        ListItem::new(result)
                    })
                    .collect(),
                2 => AudioCodec::iter()
                    .map(|codec| {
                        let mut result = String::new();
                        result += if codec == self.ffmpeg_manager.compress_settings.audio_codec {
                            "[X] "
                        } else {
                            "[ ] "
                        };
                        result += &codec.to_string();
                        ListItem::new(result)
                    })
                    .collect(),
                3 => SubtitleCodec::iter()
                    .map(|codec| {
                        let mut result = String::new();
                        result += if codec == self.ffmpeg_manager.compress_settings.subtitle_codec {
                            "[X] "
                        } else {
                            "[ ] "
                        };
                        result += &codec.to_string();
                        ListItem::new(result)
                    })
                    .collect(),
                _ => unreachable!(),
            };
            let list = List::new(items).block(settings_block).highlight_symbol(">");
            StatefulWidget::render(list, area, buf, &mut self.selected_compress_setting);
        } else if let Some(editing_string) = self.editing_string.clone() {
            let input = Paragraph::new(editing_string).block(Block::bordered().title("Input"));
            input.render(area, buf);
        } else {
            items = self
                .ffmpeg_manager
                .compress_settings
                .get_all_fields()
                .iter()
                .map(|settings| ListItem::from(settings.to_string()))
                .collect();
            let list = List::new(items).block(settings_block).highlight_symbol(">");
            StatefulWidget::render(list, area, buf, &mut self.selections[1]);
        }
    }

    fn render_files_list(&mut self, area: Rect, buf: &mut Buffer) {
        let files_block = Block::bordered()
            .title(Line::from(" Files ").centered())
            .border_set(border::ROUNDED);

        let items: Vec<ListItem> = self
            .ffmpeg_manager
            .input_files
            .iter()
            .map(|file| ListItem::from(file.path.file_name().unwrap().to_string_lossy()))
            .collect();
        let list = List::new(items).block(files_block).highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut self.selections[2]);
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

        let [main_page, command] = Layout::vertical([Min(0), Length(4)]).areas(block.inner(area));
        let command_block = Block::bordered()
            .border_set(border::ROUNDED)
            .title(Line::from(" Command ").centered());
        let ffmpeg_command = Paragraph::new(self.ffmpeg_manager.get_command_template())
            .wrap(Wrap { trim: false })
            .block(command_block);
        block.render(area, buf);
        ffmpeg_command.render(command, buf);

        self.render_settings(main_page, buf);
    }
}
