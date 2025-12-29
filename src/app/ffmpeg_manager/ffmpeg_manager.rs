use crate::app::ffmpeg_manager::compress_settings::CompressSettings;
use ratatui::widgets::ListState;
use std::fmt::Display;
use std::path::PathBuf;

#[derive(Clone)]
enum SourceType {
    Video,
    Audio,
    Subtitle,
    Other,
}

#[derive(Clone)]
pub struct Source {
    pub source_type: SourceType,
    pub codec_name: String,
    pub title: String,
}

pub struct InputFile {
    pub path: PathBuf,
    pub sources: Vec<Source>,
}

impl InputFile {
    fn from_path(path: PathBuf) -> Self {
        let sources = Self::process_path(&path);
        Self { path, sources }
    }

    fn process_path(path: &PathBuf) -> Vec<Source> {
        vec![
            Source {
                source_type: SourceType::Video,
                codec_name: "h264".to_string(),
                title: "Video".to_string(),
            },
            Source {
                source_type: SourceType::Audio,
                codec_name: "AC-3".to_string(),
                title: "Audio 1".to_string(),
            },
            Source {
                source_type: SourceType::Audio,
                codec_name: "AC-3".to_string(),
                title: "Audio 2".to_string(),
            },
        ]
    }
}

pub struct FfmpegManager {
    pub files: Vec<InputFile>,
    pub compress_settings: CompressSettings,
    pub sources: Vec<Source>,
    pub selections: [ListState; 3],
}

impl FfmpegManager {
    pub fn add_file(&mut self, path: String) {
        let input_file = InputFile::from_path(path.into());
        self.sources.extend(input_file.sources.clone());
        self.files.push(input_file);
    }

    pub fn get_selected(&self) -> Option<usize> {
        for i in 0..3 {
            if self.selections[i].selected().is_some() {
                return Some(i);
            }
        }
        None
    }
}

impl Default for FfmpegManager {
    fn default() -> Self {
        Self {
            files: Vec::new(),
            compress_settings: CompressSettings::default(),
            sources: Vec::new(),
            selections: [ListState::default(); 3],
        }
    }
}
