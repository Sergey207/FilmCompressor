use std::fmt::{Display, Formatter};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
pub enum VideoCodec {
    Libx264,
    H264Vaapi,
    HevcVaapi,
    Libsvtav1,
    Av1Vaapi,
    Copy,
}

impl Display for VideoCodec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VideoCodec::Libx264 => f.write_str("libx264"),
            VideoCodec::H264Vaapi => f.write_str("h264_vaapi"),
            VideoCodec::HevcVaapi => f.write_str("hevc_vaapi"),
            VideoCodec::Libsvtav1 => f.write_str("libsvtav1"),
            VideoCodec::Av1Vaapi => f.write_str("av1_vaapi"),
            VideoCodec::Copy => f.write_str("copy"),
        }
    }
}

impl Default for VideoCodec {
    fn default() -> Self {
        VideoCodec::Av1Vaapi
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
pub enum AudioCodec {
    Libopus,
    Copy,
}

impl Display for AudioCodec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioCodec::Libopus => f.write_str("libopus"),
            AudioCodec::Copy => f.write_str("copy"),
        }
    }
}

impl Default for AudioCodec {
    fn default() -> Self {
        AudioCodec::Libopus
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
pub enum SubtitleCodec {
    Srt,
    Ass,
    Copy,
}

impl Display for SubtitleCodec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubtitleCodec::Srt => f.write_str("srt"),
            SubtitleCodec::Ass => f.write_str("ass"),
            SubtitleCodec::Copy => f.write_str("copy"),
        }
    }
}

impl Default for SubtitleCodec {
    fn default() -> Self {
        SubtitleCodec::Ass
    }
}

#[derive(Debug, EnumIter, PartialEq, Eq, Clone)]
pub enum PixelFormat {
    Yuv420p,
    Nv10,
    Yuv420p10le,
    Copy,
}

impl Display for PixelFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PixelFormat::Yuv420p => f.write_str("yuv420p"),
            PixelFormat::Nv10 => f.write_str("nv10"),
            PixelFormat::Yuv420p10le => f.write_str("yuv420p10le"),
            PixelFormat::Copy => f.write_str("copy"),
        }
    }
}

impl Default for PixelFormat {
    fn default() -> Self {
        PixelFormat::Yuv420p10le
    }
}
