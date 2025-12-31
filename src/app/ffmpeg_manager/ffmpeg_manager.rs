use std::fmt::Display;
use crate::app::ffmpeg_manager::compress_settings::CompressSettings;
use ratatui::widgets::ListState;
use serde_json::Value;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use std::process::Command;

#[derive(Clone)]
pub struct VideoData {
    pub resolution: (u64, u64),
}

#[derive(Clone)]
pub struct AudioData {
    pub title: Option<String>,
    pub channels: u64,
    pub language: Option<String>,
}

#[derive(Clone)]
pub struct SubtitleData {
    pub title: Option<String>,
    pub language: Option<String>,
}

#[derive(Clone)]
pub enum StreamType {
    Video(VideoData),
    Audio(AudioData),
    Subtitle(SubtitleData),
    Attachment,
}

#[derive(Clone)]
pub struct Stream {
    pub stream_type: StreamType,
    pub codec_name: String,
    pub default: bool,
}

impl Display for Stream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match &self.stream_type {
            StreamType::Video(video_data) => String::from(format!(
                "Video {} ({}:{})",
                self.codec_name, video_data.resolution.0, video_data.resolution.1
            )),
            StreamType::Audio(audio_data) => {
                let mut result = String::from(format!(
                    "Audio {} ({} ch)",
                    self.codec_name, audio_data.channels
                ));
                if let Some(title) = &audio_data.title {
                    result += &format!(" {}", title);
                }
                if let Some(language) = &audio_data.language {
                    result += &format!(" {}", language);
                }
                result
            }
            StreamType::Subtitle(subtitle_data) => {
                let mut result = String::from(format!("Subtitle {}", self.codec_name));
                if let Some(title) = &subtitle_data.title {
                    result += &format!(" {}", title);
                }
                if let Some(language) = &subtitle_data.language {
                    result += &format!(" {}", language);
                }
                result
            }
            StreamType::Attachment => String::from("Attachment"),
        };
        write!(f, "{}", str)
    }
}

pub struct InputFile {
    pub path: PathBuf,
    pub sources: Vec<Stream>,
}

impl InputFile {
    fn from_path(path: PathBuf) -> Self {
        let sources = Self::process_path(&path).unwrap_or(Vec::new());
        Self { path, sources }
    }

    fn process_path(path: &PathBuf) -> Result<Vec<Stream>, Error> {
        if !path.exists() {
            return Err(ErrorKind::NotFound.into());
        }
        let output = Command::new("ffprobe")
            .arg("-show_streams")
            .arg("-output_format")
            .arg("json")
            .arg(path.to_str().expect("Failed to convert path to string"))
            .output()?;
        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                "ffprobe exited with non-zero status code",
            ));
        }

        let json_string =
            String::from_utf8(output.stdout).expect("Failed to convert output to string");
        let json_data: Value = serde_json::from_str(&json_string)?;

        let mut result = vec![];
        for stream in json_data["streams"].as_array().unwrap() {
            let new_stream = Stream {
                stream_type: match stream["codec_type"].as_str().unwrap() {
                    "video" => StreamType::Video(VideoData {
                        resolution: (
                            stream["width"].as_u64().unwrap(),
                            stream["height"].as_u64().unwrap(),
                        ),
                    }),
                    "audio" => StreamType::Audio(AudioData {
                        title: match stream["tags"]["title"].as_str() {
                            Some(title) => Some(title.to_string()),
                            None => None,
                        },
                        channels: stream["channels"].as_u64().unwrap(),
                        language: match stream["tags"]["language"].as_str() {
                            Some(language) => Some(language.to_string()),
                            None => None,
                        },
                    }),
                    "subtitle" => StreamType::Subtitle(SubtitleData {
                        title: match stream["tags"]["title"].as_str() {
                            Some(title) => Some(title.to_string()),
                            None => None,
                        },
                        language: match stream["tags"]["language"].as_str() {
                            Some(language) => Some(language.to_string()),
                            None => None,
                        },
                    }),
                    "attachment" => StreamType::Attachment,
                    &_ => {
                        continue;
                    }
                },
                codec_name: stream["codec_name"].as_str().unwrap().to_string(),
                default: stream["disposition"]["default"].as_bool().unwrap_or(false),
            };
            result.push(new_stream);
        }
        Ok(result)
    }
}

pub struct FfmpegManager {
    pub files: Vec<InputFile>,
    pub compress_settings: CompressSettings,
    pub sources: Vec<Stream>,
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
