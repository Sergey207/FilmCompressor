use crate::app::ffmpeg_manager::codec::{AudioCodec, PixelFormat, SubtitleCodec, VideoCodec};

#[derive(Debug)]
pub struct CompressSettings {
    pub video_codec: VideoCodec,
    pub pixel_format: PixelFormat,
    pub audio_codec: AudioCodec,
    pub subtitle_codec: SubtitleCodec,

    pub video_bitrate: Option<String>,
    pub audio_bitrate: Option<String>,

    pub scale: Option<String>,
    pub other_settings: String,
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

    pub fn get_init_arguments(&self) -> Vec<String> {
        let mut result = Vec::new();
        if self.video_codec.is_vaapi() {
            result.extend(vec![
                "-hwaccel".to_string(),
                "vaapi".to_string(),
                "-init_hw_device".to_string(),
                "vaapi=intel:/dev/dri/renderD128".to_string(),
            ]);
        }
        result
    }
    pub fn get_compress_arguments(&self) -> Vec<String> {
        let mut result = Vec::new();
        result.extend(vec![
            "-c:v".to_string(),
            self.video_codec.to_string(),
            "-c:a".to_string(),
            self.audio_codec.to_string(),
            "-c:s".to_string(),
            self.subtitle_codec.to_string(),
        ]);

        if self.audio_codec == AudioCodec::Libopus {
            result.extend(vec!["-ac".to_string(), "2".to_string()]);
        }

        if let Some(video_bitrate) = self.video_bitrate.clone() {
            result.extend(vec!["-b:v".to_string(), video_bitrate]);
        }
        if let Some(audio_bitrate) = self.audio_bitrate.clone() {
            result.extend(vec!["-b:a".to_string(), audio_bitrate]);
        }

        if self.video_codec.is_vaapi() || self.scale.is_some() {
            let mut video_format = String::new();
            if self.video_codec.is_vaapi() {
                if let Some(scale) = self.scale.clone() {
                    video_format = format!("scale_vaapi={},", scale);
                }
                video_format += &format!("format={},", self.pixel_format);
                video_format += "hwupload";
            } else if let Some(scale) = self.scale.clone() {
                video_format = format!("scale={}", scale);
            }
            result.extend(vec!["-vf".to_string(), video_format]);
        }

        if !self.video_codec.is_vaapi() {
            result.extend(vec!["-pix_fmt".to_string(), self.pixel_format.to_string()]);
        }

        if !self.other_settings.is_empty() {
            result.push(self.other_settings.to_string());
        }

        result
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
