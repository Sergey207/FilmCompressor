#[derive(Debug)]
pub enum VideoCodec {
    Libx264,
    H264Vaapi,
    HevcVaapi,
    Libsvtav1,
    Av1Vaapi,
}

impl VideoCodec {
    pub fn get_codec_name(&self) -> String {
        match self {
            VideoCodec::Libx264 => "libx264".to_string(),
            VideoCodec::H264Vaapi => "h264_vaapi".to_string(),
            VideoCodec::HevcVaapi => "hevc_vaapi".to_string(),
            VideoCodec::Libsvtav1 => "libsvtav1".to_string(),
            VideoCodec::Av1Vaapi => "av1_vaapi".to_string(),
        }
    }
}

impl Default for VideoCodec {
    fn default() -> Self {
        VideoCodec::Av1Vaapi
    }
}

#[derive(Debug)]
pub enum AudioCodec {
    Libopus,
}

impl AudioCodec {
    pub fn get_codec_name(&self) -> String {
        match self {
            AudioCodec::Libopus => "libopus".to_string(),
        }
    }
}

impl Default for AudioCodec {
    fn default() -> Self {
        AudioCodec::Libopus
    }
}

#[derive(Debug)]
pub enum SubtitleCodec {
    Srt,
    Ass,
}

impl SubtitleCodec {
    pub fn get_codec_name(&self) -> String {
        match self {
            SubtitleCodec::Srt => "srt".to_string(),
            SubtitleCodec::Ass => "ass".to_string(),
        }
    }
}

impl Default for SubtitleCodec {
    fn default() -> Self {
        SubtitleCodec::Ass
    }
}

#[derive(Debug)]
pub enum PixelFormat {
    Yuv420p,
    Nv10,
    Yuv420p10le,
}

impl PixelFormat {
    pub fn get_codec_name(&self) -> String {
        match self {
            PixelFormat::Yuv420p => "yuv420p".to_string(),
            PixelFormat::Nv10 => "nv10".to_string(),
            PixelFormat::Yuv420p10le => "yuv420p10le".to_string(),
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        PixelFormat::Yuv420p10le
    }
}
