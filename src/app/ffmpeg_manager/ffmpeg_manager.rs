use ratatui::widgets::ListState;
use crate::app::ffmpeg_manager::compress_settings::CompressSettings;

pub struct FfmpegManager {
    pub files: Vec<String>,
    pub selected_file: ListState,
    pub compress_settings: CompressSettings
}

impl FfmpegManager {}

impl Default for FfmpegManager {
    fn default() -> Self {
        Self {
            files: Vec::from(["File1".to_string(), "File2".to_string()]),
            selected_file: ListState::default(),
            compress_settings: CompressSettings::default()
        }
    }
}