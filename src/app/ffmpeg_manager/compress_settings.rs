use crate::app::ffmpeg_manager::codec::{AudioCodec, PixelFormat, SubtitleCodec, VideoCodec};

#[derive(Debug)]
pub struct CompressSettings {
    video_codec: VideoCodec,
    pixel_format: PixelFormat,
    audio_codec: AudioCodec,
    subtitle_codec: SubtitleCodec,

    video_bitrate: Option<String>,
    audio_bitrate: Option<String>,

    scale: Option<String>,
    other_settings: String,
}

impl CompressSettings {
    pub fn get_all_fields(&self) -> Vec<String> {
        vec![
            format!("Video codec: {}", self.video_codec.to_string()),
            format!("Pixel format: {}", self.pixel_format.to_string()),
            format!("Audio codec: {}", self.audio_codec.to_string()),
            format!("Subtitle codec: {}", self.subtitle_codec.to_string()),
            format!(
                "Video bit rate: {}",
                self.video_bitrate.clone().unwrap_or(String::from("auto"))
            ),
            format!(
                "Audio bit rate: {}",
                self.audio_bitrate.clone().unwrap_or(String::from("auto"))
            ),
            format!(
                "Scale: {}",
                self.scale.clone().unwrap_or(String::from("no"))
            ),
            format!("Other settings: {}", self.other_settings),
        ]
    }
}

impl Default for CompressSettings {
    fn default() -> Self {
        Self {
            video_codec: VideoCodec::default(),
            pixel_format: PixelFormat::default(),
            audio_codec: AudioCodec::default(),
            subtitle_codec: SubtitleCodec::default(),
            video_bitrate: None,
            audio_bitrate: None,
            scale: None,
            other_settings: String::new(),
        }
    }
}
