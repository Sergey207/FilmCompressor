use crate::app::ffmpeg_manager::codec::{AudioCodec, PixelFormat, SubtitleCodec, VideoCodec};

#[derive(Debug)]
pub enum Sources {
    All,
    Partial(Vec<u32>)
}

#[derive(Debug)]
pub struct CompressSettings {
    video_sources: Sources,
    audio_sources: Sources,
    subtitle_sources: Sources,
    default_subtitle_source: u32,

    video_codec: VideoCodec,
    pixel_format: PixelFormat,
    audio_codec: AudioCodec,
    subtitle_codec: SubtitleCodec,
    include_other_files: bool,

    video_bitrate: Option<String>,
    audio_bitrate: Option<String>,

    scale: Option<String>,
    other_settings: String,
}

impl Default for CompressSettings {
    fn default() -> Self {
        Self {
            video_sources: Sources::All,
            audio_sources: Sources::All,
            subtitle_sources: Sources::All,
            default_subtitle_source: 0,
            video_codec: VideoCodec::default(),
            pixel_format: PixelFormat::default(),
            audio_codec: AudioCodec::default(),
            subtitle_codec: SubtitleCodec::default(),
            include_other_files: false,
            video_bitrate: None,
            audio_bitrate: None,
            scale: None,
            other_settings: String::new(),
        }
    }
}